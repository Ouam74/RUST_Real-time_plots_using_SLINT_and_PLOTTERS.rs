// #![windows_subsystem = "windows"]

use std::{thread, time::Duration, sync::Arc, sync::Mutex, sync::mpsc::channel, sync::mpsc::sync_channel};
use rand::Rng;
use plotters::prelude::*;
use plotters::{backend::BitMapBackend, chart::ChartBuilder, series::LineSeries};
use slint::{RgbaColor, Color, Image, SharedPixelBuffer, Brush, SharedString};


slint::slint! {
    import { MainWindow } from "src/Thread3.slint";
}

static _N: i64 = 10000;

fn createserie_iter() -> Vec<(f64, f64)> {
    let mut ret = vec![];
    let mut _rng = rand::thread_rng();
    for j in 0.._N {
        ret.push( ( j as f64, _rng.gen::<f64>() ) );
    }
    ret
}

fn render_plot(newserie: Vec<(f64, f64)>) -> Image {
    let mut pixel_buffer = SharedPixelBuffer::new(800, 480);
    // let mut pixel_buffer = slint::SharedPixelBuffer::new(1600, 960);
    let size = (pixel_buffer.width(), pixel_buffer.height());
    let backend = BitMapBackend::with_buffer(pixel_buffer.make_mut_bytes(), size);

    // Plotters requires TrueType fonts from the file system to draw axis text - we skip that for WASM for now. 
    #[cfg(target_arch = "wasm32")]
    let backend = slint::wasm_backend::BackendWithoutText { backend };
    
    let root = backend.into_drawing_area();

    root.fill(&WHITE).expect("error filling drawing area");

    let mut chart = ChartBuilder::on(&root)
        .build_cartesian_2d(0.0.._N as f64, 0.0..1.0)
        .unwrap();

    chart.configure_mesh().draw().unwrap();

    // chart.draw_series(plotters::series::LineSeries::new(createserie_iter(), &RED).point_size(0))
    //      .expect("error drawing series");

    chart.draw_series(LineSeries::new(newserie, &RED).point_size(0))
    .expect("error drawing series");

    root.present().expect("error presenting");
    drop(chart);
    drop(root);

    Image::from_rgb8(pixel_buffer)
}

pub fn main() {
    
    let ui = MainWindow::new();
    let ui_handle = ui.as_weak();
    let abort = Arc::new(Mutex::new(0));
    let (tx, rx) = channel();

    // Plot empty image..
    let handle_copy = ui_handle.clone();
    let mut empty_serie = vec![];
    for j in 0.._N {
        empty_serie.push( ( j as f64, 0.0 ) );
    }
    slint::invoke_from_event_loop(move || {
        handle_copy.unwrap().set_new_image(render_plot(empty_serie));
    });
    
    let abort_clone = abort.clone(); 
    ui.on_start_clicked({
        move || {
            let tx = tx.clone();

            let abort_clone = abort.clone(); 
            *abort_clone.lock().unwrap() = 0;
            
            let ui_handle_clone = ui_handle.clone(); 
            let abort_clone = abort.clone(); 
            let _thread1 = thread::spawn(move || {
                loop {
                    if *abort_clone.lock().unwrap() == 0 {
                        let newserie = createserie_iter();
                        tx.send(newserie).unwrap();
                        thread::sleep(Duration::from_millis(10));
                    }
                    else {
                        break;
                    };
                };
                let handle_copy = ui_handle_clone.clone();
                slint::invoke_from_event_loop(move || {
                    handle_copy.unwrap().set_start_status(false);
                }).err();
            });

            let abort_clone = abort.clone(); 
            let ui_handle_clone = ui_handle.clone();
            let _thread2 = std::thread::spawn(move || {
                loop {
                    if *abort_clone.lock().unwrap() == 0 {
                            thread::sleep(Duration::from_millis(500));
                            let handle_copy = ui_handle_clone.clone();
                            slint::invoke_from_event_loop(move || {
                                handle_copy.unwrap().set_current_color(Brush::SolidColor(Color::from(RgbaColor{ red: 0.0, green: 0.0, blue: 1.0, alpha: 1.})));
                                handle_copy.unwrap().set_current_string(SharedString::from("RUNNING"));
                            }).err();
                    }
                    if *abort_clone.lock().unwrap() == 0 {
                        thread::sleep(Duration::from_millis(500));
                        let handle_copy = ui_handle_clone.clone();
                        slint::invoke_from_event_loop(move || {
                            handle_copy.unwrap().set_current_color(Brush::SolidColor(Color::from(RgbaColor{ red: 1.0, green: 0.5, blue: 0.0, alpha: 1.})));
                            handle_copy.unwrap().set_current_string(SharedString::from("RUNNING"));
                        }).err();
                    }   
                    else {
                        let handle_copy = ui_handle_clone.clone();
                        slint::invoke_from_event_loop(move || {
                            handle_copy.unwrap().set_current_color(Brush::SolidColor(Color::from(RgbaColor{ red: 0.0, green: 0.0, blue: 0.0, alpha: 0.0})));
                            handle_copy.unwrap().set_current_string(SharedString::from("       "));
                        }).err();
                        break;
                    };
                }
            });
        }
    });
    
    let ui_handle = ui.as_weak();
    std::thread::spawn(move || {
        for received in rx {
            let handle_copy = ui_handle.clone();
            slint::invoke_from_event_loop(move || {
                handle_copy.unwrap().set_new_image(render_plot(received));
            }).err();
            thread::sleep(Duration::from_millis(10));
        };
    });
    
    let abort_clone = abort_clone.clone(); // let abort_clone = Arc::clone(&abort_clone);
    ui.on_stop_clicked({
        move || {
            *abort_clone.lock().unwrap() = 1;
        }
    });
    
    ui.run();
}


// use std::{thread, time};
// use std::sync::{Arc, Mutex};
// use rand::Rng;
// use plotters::prelude::*;
// use slint::{Image, RgbaColor, Color, SharedPixelBuffer, Brush, SharedString};


// slint::slint! {
//     import { MainWindow } from "src/Thread3.slint";
// }

// static _N: i64 = 10000;

// fn createserie_iter() -> Vec<(f64, f64)> {
//     let mut ret = vec![];
//     let mut _rng = rand::thread_rng();
//     for j in 0.._N {
//         ret.push( ( j as f64, _rng.gen::<f64>() ) );
//     }
//     ret
// }

// fn render_plot(newserie: Vec<(f64, f64)>) -> Image {
//     let mut pixel_buffer = SharedPixelBuffer::new(800, 480);
//     let size = (pixel_buffer.width(), pixel_buffer.height());
//     let backend = plotters::backend::BitMapBackend::with_buffer(pixel_buffer.make_mut_bytes(), size);

//     // Plotters requires TrueType fonts from the file system to draw axis text - we skip that for WASM for now. 
//     #[cfg(target_arch = "wasm32")]
//     let backend = slint::wasm_backend::BackendWithoutText { backend };
    
//     let root = backend.into_drawing_area();

//     root.fill(&WHITE).expect("error filling drawing area");

//     let mut chart = plotters::chart::ChartBuilder::on(&root)
//         .build_cartesian_2d(0.0.._N as f64, 0.0..1.0)
//         .unwrap();

//     chart.configure_mesh().draw().unwrap();

//     chart.draw_series(plotters::series::LineSeries::new(newserie, &RED).point_size(0))
//     .expect("error drawing series");

//     root.present().expect("error presenting");
//     drop(chart);
//     drop(root);

//     Image::from_rgb8(pixel_buffer)
// }


// pub fn main() {
    
//     let ui = MainWindow::new();
//     let ui_handle = ui.as_weak();
//     let abort = Arc::new(Mutex::new(0));
    
//     // Plot empty image..
//     let handle_copy = ui_handle.clone();
//     let mut empty_serie = vec![];
//     for j in 0.._N {
//         empty_serie.push( ( j as f64, 0.0 ) );
//     }
//     slint::invoke_from_event_loop(move || {
//         handle_copy.unwrap().set_new_image(render_plot(empty_serie));
//     });
    
//     let ui_handle_clone = ui_handle.clone();
//     let abort_clone = abort.clone(); 
//     ui.on_start_clicked({
//         move || {
//             *abort.lock().unwrap() = 0;

//             let ui_handle_clone = ui_handle.clone(); 
//             let abort_clone = abort.clone(); 
//             let thread1 = std::thread::spawn(move || {
//                 loop {
//                     let newserie = createserie_iter();
//                     thread::sleep(time::Duration::from_millis(1));
//                     if *abort_clone.lock().unwrap() == 0 {
//                         let handle_copy = ui_handle_clone.clone();
//                         slint::invoke_from_event_loop(move || {
//                             handle_copy.unwrap().set_start_status(true);
//                             handle_copy.unwrap().set_new_image(render_plot(newserie));
//                         });
//                     }
//                     else {
//                         // println!("abort_clone (thread1) = {}", abort_clone.lock().unwrap());
//                         break;
//                     };
//                 };
//                 let handle_copy = ui_handle_clone.clone();
//                 slint::invoke_from_event_loop(move || {
//                     handle_copy.unwrap().set_start_status(false);
//                 });
//             });
            
//             let ui_handle_clone = ui_handle.clone();
//             let abort_clone = abort.clone();  
//             let thread2 = std::thread::spawn(move || {
//                 loop {
//                     if *abort_clone.lock().unwrap() == 0 {
//                             let handle_copy = ui_handle_clone.clone();
//                             thread::sleep(time::Duration::from_millis(500));
//                             slint::invoke_from_event_loop(move || {
//                                 handle_copy.unwrap().set_current_color(Brush::SolidColor(Color::from(RgbaColor{ red: 0.0, green: 0.0, blue: 1.0, alpha: 1.})));
//                                 handle_copy.unwrap().set_current_string(SharedString::from("RUNNING"));
//                             });
//                     }
//                     if *abort_clone.lock().unwrap() == 0 {
//                         let handle_copy = ui_handle_clone.clone();
//                         thread::sleep(time::Duration::from_millis(500));
//                         slint::invoke_from_event_loop(move || {
//                             handle_copy.unwrap().set_current_color(Brush::SolidColor(Color::from(RgbaColor{ red: 1.0, green: 0.5, blue: 0.0, alpha: 1.})));
//                             handle_copy.unwrap().set_current_string(SharedString::from("RUNNING"));
//                         });
//                     }   
//                     else {
//                         // println!("abort_clone (thread2) = {}", abort_clone.lock().unwrap());
//                         break;
//                     };
//                 }
//                 let handle_copy = ui_handle_clone.clone();
//                 slint::invoke_from_event_loop(move || {
//                     handle_copy.unwrap().set_current_color(Brush::SolidColor(Color::from(RgbaColor{ red: 0.0, green: 0.0, blue: 0.0, alpha: 0.0})));
//                     handle_copy.unwrap().set_current_string(SharedString::from("       "));
//                 });
//             });
//             // thread1.join().unwrap();
//             // thread2.join().unwrap();
//         }
//     });
    
//     let abort_clone = abort_clone.clone(); // let abort_clone = Arc::clone(&abort_clone);
//     ui.on_stop_clicked({
//         move || {
//             *abort_clone.lock().unwrap() = 1;
//             // println!("abort_clone = {}", abort_clone.lock().unwrap());
//         }
//     });
    
//     ui.run();
// }

