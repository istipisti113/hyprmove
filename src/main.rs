use clap::{Parser, Subcommand, ValueEnum};
use std::str::FromStr;
use crate::monitor::get_by_id;
use std::env;

use std::process::Command;

mod hyprland_ipc;
use crate::workspace::*;

use hyprland::{
    data::{Client, Monitor, Transforms},
    dispatch::Direction,
};

use hyprland::data::*;
use hyprland::prelude::*;
use hyprland::shared::HResult;

use hyprland_ipc::{client, monitor, option, workspace};

pub fn get_current_monitor() -> Monitor {
    monitor::get().find(|m| m.focused).unwrap()
}

fn detect_order() -> Vec<i16>{
    let mut order : Vec<Monitor> = Vec::new();
    for i in monitor::get() {
        order.push(i)
    } 
    order.sort_by_key(|a| a.x);
    let ordered_ids : Vec<i16 >= order.iter().map(|mon| mon.id as i16).collect();
    return ordered_ids;
}

fn get_target_old(direction: &str, current: i32) -> i32 {
    let mut id = 1;
    if direction == "l" {
        if current == 2{
            id = get_by_id(0).active_workspace.id;
        } else if current == 0 {
            id = get_by_id(1).active_workspace.id;
        }
    } else if direction == "r" {
        if current == 1 {
            id = get_by_id(0).active_workspace.id;
        } else if current == 0 {
            id = get_by_id(2).active_workspace.id;
        }
    }
    return id
}

fn get_target(order: Vec<i16>, current: i16, direction: &str) -> i16 {
	for i in 0..order.len() {
        if current == order[i] {
            match direction {
                "l" => {
                    if i == 0 {
                        let target = get_by_id(order[usize::try_from(i).unwrap()]).active_workspace.id.try_into().unwrap();
                        println!("target {} current {}", target, current);
                        return target
                    } else {
                        let target = get_by_id(order[usize::try_from(i-1).unwrap()]).active_workspace.id.try_into().unwrap();
                        println!("target {} current {}", target, current);
                    	return target
                    }
                },
                "r" => {
                    if i == order.len()-1 {
                        let target = get_by_id(order[usize::try_from(i).unwrap()]).active_workspace.id.try_into().unwrap();
                        println!("target {} current {}", target, current);
                        return target
                    } else {
                        let target = get_by_id(order[usize::try_from(i+1).unwrap()]).active_workspace.id.try_into().unwrap();
                        println!("target {} current {}", target, current);
                        return target
                    }
                }
                _ => return 0
                // Some(_) => return 0,
                // None => return 0,
            }
        }
    }
    return 0
}

fn main() {
    let args: Vec<_> = env::args().collect();
    let order = detect_order();

    let current = get_current_monitor().id;
    match args[1].as_str() {
        "c" => {
			println!("{}", current);
        },
        "l" | "r" => {
       		// move the focus to the left monitor
       		let id = get_target(order, current.try_into().unwrap(), &args[1]);
            focus(&(id as u64));
        },
        "m" | "catch" => {
        	// move the focused window to the left or right monitor and follow it
        	if args[2] == "r" || args[2] == "l" {
        	    let id = get_target(order, current.try_into().unwrap(), &args[2]);
        	    // Command::new("hyprctl").arg(format!("dispatch movetoworkspace {id}")).output().expect("fasz").stdout.as_slice();
                move_focus(&(id as u64));
        	}
        },
        "n" | "throw" => {
        	// move the focused window to the left or right monitor but dont follow it
        	if args[2] == "r" || args[2] == "l" {
        	    // let old_id = get_by_id(current).active_workspace.id;
        	    let id = get_target(order, current.try_into().unwrap(), &args[2]);
        	    // Command::new("hyprctl").arg(format!("dispatch movetoworkspace {id}")).output().expect("fasz").stdout.as_slice();
        	    // Command::new("hyprctl").arg(format!("dispatch workspace {old_id}")).output().expect("fasz").stdout.as_slice();
                move_to(&(id as u64));
        	}
        },
        "w" | "pull" => {
        	// move the focused window to the next of prev workspace on that monitor and follow it
        	if args[2] == "r" || args[2] == "l" {
        	    let mut id = get_by_id(current.try_into().unwrap()).active_workspace.id;
        	    if ((current)*10+1..current*10+11).contains(&(id as i128)) {
        	        if args[2] == "r" {
        	            id = id+1
        	        } else {
        	            id = id-1
        	        }
        	        // Command::new("hyprctl").arg(format!("dispatch movetoworkspace {id}")).output().expect("fasz").stdout.as_slice();
                	move_focus(&(id as u64));
        	    }
        	}
        },
		"b" | "push" => {
            // move the window to the next or prev workspace on the current monitor and dont follow
            // it with the focus
        	if args[2] == "r" || args[2] == "l" {
        	    let mut id = get_by_id(current as i16).active_workspace.id;
        	    // let old_id = id;
        	    if ((current)*10+1..current*10+11).contains(&(id as i128)) {
        	        if args[2] == "r" {
        	            id = id+1
        	        } else {
        	            id = id-1
        	        }
        	        // Command::new("hyprctl").arg(format!("dispatch movetoworkspace {id}")).output().expect("fasz").stdout.as_slice();
        	        // Command::new("hyprctl").arg(format!("dispatch workspace {old_id}")).output().expect("fasz").stdout.as_slice();
                	move_to(&(id as u64));
        	    }
        	}
        },
        "replace" => {
			// replaces the clients on two workspaces on two monitors
            if args[2] == "r" || args[2] == "l" {
                let id = get_target(order, current.try_into().unwrap(), &args[2]);
				let mut  current_clients: Vec<Client> = Vec::new();
				let mut target_clients: Vec<Client> = Vec::new();

                let clients = Clients::get().unwrap();
                for i in clients {
                    if i.workspace.id == id as i32 {
						target_clients.push(i)
                    } else if i.workspace.id == get_by_id(current.try_into().unwrap()).active_workspace.id as i32 {
                        current_clients.push(i)
                    }
                }
                println!("targets {:?}", target_clients);
                println!("currents {:?}", current_clients);
                for mut i in target_clients{
                    i.workspace.id = get_by_id(current.try_into().unwrap()).active_workspace.id as i32;
                }
            }
        },
        "d" => {
            println!("{:?}", detect_order());
        },
        _ => {}
    }
}
