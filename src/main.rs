use std::{thread, time};
use std::sync::mpsc::{self, channel, sync_channel, Receiver, Sender, TryRecvError};
use std::sync::{Arc, Mutex};
use rand::Rng;
use plotters::prelude::*;
use slint::SharedPixelBuffer;

slint::slint! {
    import { MainWindow } from "src/Thread3.slint";
}

static _N: i64 = 1000;

fn createserie_iter() -> Vec<(f64, f64)> {
    let mut ret = vec![];
    let mut _rng = rand::thread_rng();
    for j in 0.._N {
        ret.push( ( j as f64, _rng.gen::<f64>() ) );
    }
    ret
}


fn render_plot() -> slint::Image {
    let mut pixel_buffer = SharedPixelBuffer::new(640, 480);
    let size = (pixel_buffer.width(), pixel_buffer.height());

    let backend = BitMapBackend::with_buffer(pixel_buffer.make_mut_bytes(), size);

    // Plotters requires TrueType fonts from the file system to draw axis text - we skip that for WASM for now. 
    #[cfg(target_arch = "wasm32")]
    let backend = wasm_backend::BackendWithoutText { backend };

    let root = backend.into_drawing_area();

    root.fill(&WHITE).expect("error filling drawing area");

    let mut chart = ChartBuilder::on(&root)
        .build_cartesian_2d(0.0.._N as f64, 0.0..1.0)
        .unwrap();

    chart.configure_mesh().draw().unwrap();

    chart.draw_series(LineSeries::new(createserie_iter(), &RED).point_size(0))
         .expect("error drawing series");

    root.present().expect("error presenting");
    drop(chart);
    drop(root);

    slint::Image::from_rgb8(pixel_buffer)
}


pub fn main() {
    
    let ui = MainWindow::new();
    let ui_handle = ui.as_weak();
    let abort = Arc::new(Mutex::new(0)); 
    
    let abort_clone = abort.clone(); 
    ui.on_start_clicked({
        move || {
            println!("START");
            println!("{:}", abort_clone.lock().unwrap());
            let ui = ui_handle.unwrap();
            let ui_handle = ui.as_weak();
            
            let abort_clone = abort_clone.clone(); 
            std::thread::spawn(move || {
                // ... Do some computation in the thread
                for _ in 0..100 {
                    if *abort_clone.lock().unwrap() == 0 {
                        thread::sleep(time::Duration::from_millis(100));
                        // now forward the data to the main thread using invoke_from_event_loop
                        let handle_copy = ui_handle.clone();
                        slint::invoke_from_event_loop(move || {
                            handle_copy.unwrap().set_start_status(true);
                            handle_copy.unwrap().set_new_image(render_plot());
                        });
                    }
                    else {
                        println!("out");
                        break;
                    };
                };
                let handle_copy = ui_handle.clone();
                slint::invoke_from_event_loop(move || {
                    handle_copy.unwrap().set_start_status(false);
                });
            });
        }
    });
    
    let abort_clone = abort.clone(); 
    ui.on_stop_clicked({
        move || {
            println!("STOP");
            *abort_clone.lock().unwrap() = 1;
            println!("{:}", abort_clone.lock().unwrap());
        }
    });
    
    ui.run();
}
