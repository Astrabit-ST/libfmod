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
static SENDER: OnceCell<Sender<Callback>> = OnceCell::new();

pub fn send(callback: impl FnOnce(&magnus::Ruby) + Send + 'static) {
    let sender = SENDER.get().expect("callback sender not initialized");
    sender
        .send(Box::new(callback))
        .expect("calling thread is dead. please report this");
}

pub fn bind() {
    let (sender, reciever) = std::sync::mpsc::channel();
    SENDER.set(sender).unwrap();

    unsafe {
        thread::spawn_ruby_thread(move |ruby| {
            thread::without_gvl(|| {
                while let Ok(callback) = reciever.recv() {
                    thread::with_gvl(|ruby| {
                        callback(ruby);
                        while let Ok(callback) = reciever.try_recv() {
                            callback(ruby);
                        }
                    })
                }
            });

            ruby.qnil().as_value()
        });
    }
}
