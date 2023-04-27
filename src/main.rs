#![feature(async_closure)]
mod camera;
mod constants;
mod hittable;
mod payload;
mod random;
mod ray;
mod scatterable;
mod sphere;
mod vec3;
mod world;

use crate::camera::Camera;
use crate::constants::{ASPECT_RATIO, IMAGE_HEIGHT, IMAGE_WIDTH, MAX_DEPTH, SAMPLES_PER_PIXEL};
use crate::hittable::Hittable;
use crate::payload::{BigBoy, Payload};
use crate::random::random_f32;
use crate::ray::Ray;

use crate::vec3::Color;
use crate::world::World;
use axum::extract::{
    ws::{Message, WebSocket, WebSocketUpgrade},
    State,
};
use axum::http::Response;
use axum::response::{Html, IntoResponse};
use axum::{routing::get, Router, Server};
use futures::{sink::SinkExt, stream::StreamExt};
use rayon::prelude::*;
use serde::Deserialize;
use std::io::{BufWriter, Error, Write};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::time::Instant;
use std::{fs::File, sync::Mutex};
use tokio::{sync::broadcast, task};

fn ray_color(r: Ray, world: &World, depth: i32) -> Color {
    if depth <= 0 {
        return Color::diagonal(0.0);
    }
    if let Some(hit_record) = world.hit(r, 0.001, std::f32::INFINITY) {
        return match hit_record.material.scatter(r, &hit_record) {
            Some((scattered, attenuation)) => attenuation * ray_color(scattered, world, depth - 1),
            None => Color::new(0.0, 0.0, 0.0),
        };
    }

    let unit_direction = r.direction.unit_vector();
    let t = 0.5 * (unit_direction.y() + 1.0);
    // filling in sky color
    (1.0 - t) * Color::diagonal(1.0) + t * Color::new(0.5, 0.7, 1.0)
}

async fn root_get() -> impl IntoResponse {
    let markup = tokio::fs::read_to_string("./www/index.html").await.unwrap();
    Html(markup)
}

async fn indexmjs_get() -> impl IntoResponse {
    let index = tokio::fs::read_to_string("./www/index.mjs").await.unwrap();
    Response::builder()
        .header("content-type", "application/javascript;charset=utf-8")
        .body(index)
        .unwrap()
}

async fn css_get() -> impl IntoResponse {
    let css = tokio::fs::read_to_string("./www/style.css").await.unwrap();
    Response::builder()
        .header("content-type", "text/css;charset=utf-8")
        .body(css)
        .unwrap()
}

async fn websocket_get(
    ws: WebSocketUpgrade,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    ws.on_upgrade(|ws: WebSocket| websocket(ws, state))
}

async fn websocket(stream: WebSocket, state: Arc<AppState>) {
    let (mut sender, mut receiver) = stream.split();
    let mut rx = state.tx.subscribe();
    println!("Connection established.");
    let send_task = tokio::spawn(async move {
        while let Ok(msg) = rx.recv().await {
            if sender.send(Message::Text(msg)).await.is_err() {
                break;
            }
        }
    });

    let recv_task = tokio::spawn(async move {
        while let Some(Ok(msg)) = receiver.next().await {
            let render_settings = &mut state.render_settings.lock().unwrap();
            let message: Result<RenderSettings, _> = serde_json::from_str(msg.to_text().unwrap());
            let settings = match message() {
                RenderSettings => message.unwrap(),
                Err(e) => {println!("{:?}", e); panic!("oh no {}", e);},
            }
            // if let Ok(settings) =
            //     serde_json::from_str::<Result<serde_json::Value, ()>>(msg.to_text().unwrap())
            //         .unwrap()
            // {
            //     println!("butt butt butt");
            if msg.into_text().unwrap() == "beep" {
                let render_state = state.clone();
                task::spawn_blocking(move || {
                    render(
                        &render_state,
                        vec![0; (IMAGE_WIDTH * IMAGE_HEIGHT * 3.0) as usize],
                    );
                });
            }
        }
    });
}

struct AppState {
    tx: broadcast::Sender<String>,
    pixels: Arc<Mutex<Vec<u8>>>,
    render_settings: Arc<Mutex<RenderSettings>>,
}
#[derive(Deserialize)]
struct RenderSettings {
    width: f32,
    samples: f32,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let (tx, _rx) = broadcast::channel(10);
    let image = vec![0; (IMAGE_WIDTH * IMAGE_HEIGHT * 3.0) as usize];
    let app_state = Arc::new(AppState {
        tx,
        pixels: Arc::new(Mutex::new(image)),
        render_settings: Arc::new(Mutex::new(RenderSettings {
            width: IMAGE_WIDTH,
            samples: SAMPLES_PER_PIXEL,
        })),
    });
    let router = Router::new()
        .route("/", get(root_get))
        .route("/index.mjs", get(indexmjs_get))
        .route("/style.css", get(css_get))
        .route("/ws", get(websocket_get))
        .with_state(app_state.clone());

    let server = Server::bind(&"0.0.0.0:7032".parse().unwrap()).serve(router.into_make_service());
    let local_addr = server.local_addr();
    println!("Listening on {}", local_addr);

    server.await.unwrap();

    Ok(())
}

fn render(state: &Arc<AppState>, image: Vec<u8>) -> impl IntoResponse {
    let camera = Camera::new();
    let bands: Vec<(usize, &[u8])> = image
        .chunks((IMAGE_WIDTH * 3.0) as usize)
        .enumerate()
        .collect();
    let world = World::default();

    // Render here
    static ELAPSED: AtomicUsize = AtomicUsize::new(0);
    let start = Instant::now();
    bands.into_par_iter().for_each(|(i, _band)| {
        for x in 0..IMAGE_WIDTH as usize {
            let mut pixel_color = Color::default();
            for _s in 0..SAMPLES_PER_PIXEL as i32 {
                let u = (x as f32 + random_f32()) / (IMAGE_WIDTH - 1.0);
                let v = (IMAGE_HEIGHT - (i as f32 + random_f32())) / (IMAGE_HEIGHT - 1.0);
                let r = camera.get_ray(u, v);
                pixel_color += ray_color(r, &world, MAX_DEPTH);
            }
            let (r, g, b) = pixel_color.color(SAMPLES_PER_PIXEL);
            let image = &mut state.pixels.lock().unwrap();
            let index = x * 3 + i * 3 * IMAGE_WIDTH as usize;
            image[index] = r;
            image[index + 1] = g;
            image[index + 2] = b;
        }
        let elapsed_count = ELAPSED.fetch_add(1, Ordering::SeqCst) + 1;
        let left_bound = i * 3 * IMAGE_WIDTH as usize;
        let right_bound = left_bound + 3 * IMAGE_WIDTH as usize;
        let payload = Payload {
            row: i,
            pixels: state.pixels.lock().unwrap()[left_bound..right_bound]
                .try_into()
                .unwrap(),
        };
        let payload = serde_json::to_string(&payload).unwrap();
        state.tx.send(payload).unwrap();
        if elapsed_count % 50 == 0 || elapsed_count == 168 {
            println!("{}/{}", elapsed_count, IMAGE_HEIGHT);
        }
    });
    state.tx.send(format!("end")).unwrap();
    println!("Frame time: {}ms", start.elapsed().as_millis());

    let f = File::create("./image.ppm").expect("Unable to create file.");
    let mut f = BufWriter::new(f);
    let _ = f.write(format!("P3\n{IMAGE_WIDTH} {IMAGE_HEIGHT}\n255\n").as_bytes());

    state.pixels.lock().unwrap().chunks(3).for_each(|color| {
        let _ = f.write(format!("{} {} {}\n", color[0], color[1], color[2]).as_bytes());
    });
}
