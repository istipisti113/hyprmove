use clap::{Parser, Subcommand, ValueEnum};
use crate::monitor::get_by_id;
use std::env;

use std::process::Command;

mod hyprland_ipc;
use hyprland::{
    data::{Client, Monitor, Transforms},
    dispatch::Direction,
};

use hyprland_ipc::{client, monitor, option, workspace};

pub fn get_current_monitor() -> Monitor {
    monitor::get().find(|m| m.focused).unwrap()
}

fn get_target(direction: &str, current: i32) -> i32 {
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

fn main() {
    let args: Vec<_> = env::args().collect();

	if args[1] == "custom"{
        let current = get_current_monitor().id;
        if args[2] == "c" { // for current
			println!("{} {}", get_by_id(current).active_workspace.id, current);

        // move the focus to the left monitor
        } else if args[2] == "l" || args[2] == "r" {
            let id = get_target(&args[2], current.into());
			Command::new("hyprctl").arg(format!("dispatch workspace {id}")).output().expect("fasz").stdout.as_slice();
    	
        // move the focused window to the left or right monitor and follow it
        } else if args[2] == "m" {
			if args[3] == "r" || args[3] == "l" {
                let id = get_target(&args[3], current.into());
                Command::new("hyprctl").arg(format!("dispatch movetoworkspace {id}")).output().expect("fasz").stdout.as_slice();
            }
        // move the focused window to the left or right monitor but dont follow it
    	} else if args[2] == "n" {
			if args[3] == "r" || args[3] == "l" {
                let old_id = get_by_id(current).active_workspace.id;
                let id = get_target(&args[3], current.into());
                Command::new("hyprctl").arg(format!("dispatch movetoworkspace {id}")).output().expect("fasz").stdout.as_slice();
                Command::new("hyprctl").arg(format!("dispatch workspace {old_id}")).output().expect("fasz").stdout.as_slice();
            }
		// move the focused window to the next of prev workspace on that monitor and follow it
        } else if args[2] == "w" {
			if args[3] == "r" || args[3] == "l" {
                let mut id = get_by_id(current).active_workspace.id;
                if ((current)*10+1..current*10+11).contains(&(id as i16)) {
                    if args[3] == "r" {
                        id = id+1
                    } else {
                        id = id-1
                    }
                	Command::new("hyprctl").arg(format!("dispatch movetoworkspace {id}")).output().expect("fasz").stdout.as_slice();
                }
            }
        } else if args[2] == "b" {
			if args[3] == "r" || args[3] == "l" {
                let mut id = get_by_id(current).active_workspace.id;
                let old_id = id;
                if ((current)*10+1..current*10+11).contains(&(id as i16)) {
                    if args[3] == "r" {
                        id = id+1
                    } else {
                        id = id-1
                    }
                	Command::new("hyprctl").arg(format!("dispatch movetoworkspace {id}")).output().expect("fasz").stdout.as_slice();
                	Command::new("hyprctl").arg(format!("dispatch workspace {old_id}")).output().expect("fasz").stdout.as_slice();
                }
            }
        }
        return
    }
}
