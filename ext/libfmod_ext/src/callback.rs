// Copyright (c) 2024 Lily Lyons
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use magnus::prelude::*;
use once_cell::sync::OnceCell;
use std::sync::mpsc::Sender;

use crate::thread;

type Callback = Box<dyn FnOnce(&magnus::Ruby) + Send>;
static SENDER: OnceCell<Sender<Option<Callback>>> = OnceCell::new();

pub fn process(callback: impl FnOnce(&magnus::Ruby) + Send + 'static) {
    if let Ok(ruby) = magnus::Ruby::get() {
        callback(&ruby);
        return;
    }
    let sender = SENDER.get().expect("callback sender not initialized");
    sender
        .send(Some(Box::new(callback)))
        .expect("calling thread is dead. please report this");
}

pub fn bind() {
    let (sender, reciever) = std::sync::mpsc::channel();
    SENDER.set(sender).unwrap();

    unsafe {
        thread::spawn_ruby_thread(move |ruby| {
            thread::without_gvl(
                || {
                    // let mut running = true;
                    // while running {
                    loop {
                        let Ok(Some(callback)) = reciever.recv() else {
                            return;
                        };
                        thread::with_gvl(|ruby| {
                            callback(ruby);
                            // for callback in reciever.try_iter() {
                            //    let Some(callback) = callback else {
                            //        running = false;
                            //        break;
                            //    };
                            //    callback(ruby);
                            // }
                        });
                    }
                },
                || {
                    // we send None to signal that the thread is done
                    let _ = SENDER.get().unwrap().send(None);
                },
            );

            ruby.qnil().as_value()
        });
    }
}
