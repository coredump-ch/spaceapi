//! Coredump implementation of the Space API server.
//!
//! Start this via the commandline:
//!
//!     ./coredump_status [-p PORT] [-i IP]
extern crate env_logger;
extern crate docopt;
extern crate rustc_serialize;
extern crate spaceapi_server;

mod utils;

use std::process::exit;
use docopt::Docopt;
use spaceapi_server::SpaceapiServer;
use spaceapi_server::api;
use spaceapi_server::api::sensors::{TemperatureSensorTemplate, PeopleNowPresentSensorTemplate};
use spaceapi_server::api::Optional::{Value, Absent};
use utils::Ipv4;


static USAGE: &'static str = "
Usage: coredump_status [-p PORT] [-i IP]

Options:
    -p PORT  The port to listen on [default: 3000].
    -i IP    The ipv4 address to listen on [default: 127.0.0.1].
";

#[derive(RustcDecodable, Debug)]
struct Args {
    flag_p: u16,
    flag_i: Ipv4,
}

#[cfg_attr(test, allow(dead_code))]
fn main() {
    env_logger::init().unwrap();

    // Parse arguments
    let args: Args = Docopt::new(USAGE).and_then(|d| d.decode())
                                       .unwrap_or_else(|e| e.exit());
    let host = args.flag_i.ip;
    let port = args.flag_p;

    // Create new Status instance
    let mut status = api::Status::new(
        "coredump",
        "https://www.coredump.ch/logo.png",
        "https://www.coredump.ch/",
        api::Location {
            address: Value("Zürcherstrasse 6, 8640 Rapperswil, Switzerland".into()),
            lat: 47.22939,
            lon: 8.82041,
        },
        api::Contact {
            irc: Value("irc://freenode.net/#coredump".into()),
            twitter: Value("@coredump_ch".into()),
            email: Value("vorstand@lists.coredump.ch".into()),
            foursquare: Absent,
        },
        vec![
            "email".into(),
            "twitter".into(),
        ],
    );

    // Add optional data
    status.spacefed = Value(api::Spacefed {
        spacenet: false,
        spacesaml: false,
        spacephone: false,
    });
    status.feeds = Value(api::Feeds {
        blog: Value(api::Feed {
            _type: Value("rss".into()),
            url: "https://www.coredump.ch/feed/".into(),
        }),
        wiki: Absent,
        calendar: Absent,
        flickr: Absent,
    });
    status.projects = Value(vec![
        "https://www.coredump.ch/projekte/".into(),
        "https://forum.coredump.ch/c/projects".into(),
        "https://github.com/coredump-ch/".into(),
    ]);
    status.cam = Value(vec![
        "https://webcam.coredump.ch/cams/ultimaker_0.jpg".into(),
        "https://webcam.coredump.ch/cams/ultimaker_1.jpg".into(),
        "https://webcam.coredump.ch/cams/ultimaker_2.jpg".into(),
    ]);
    status.state.message = Value("Open Mondays from 20:00".into());

    // Set up datastore
    let redis_url = "redis://127.0.0.1/";

    // Set up modifiers
    let modifiers = Vec::new();

    // Set up server
    let mut server = SpaceapiServer::new((host, port), status, redis_url, modifiers)
        .unwrap_or_else(|e| {
            println!("Could not initialize server: {:?}", e);
            exit(1);
        });

    // Register sensors
    server.register_sensor(Box::new(TemperatureSensorTemplate {
        unit: "°C".into(),
        location: "Hackerspace".into(),
        name: Value("Raspberry CPU".into()),
        description: Absent,
    }), "temperature_raspi".into());
    server.register_sensor(Box::new(TemperatureSensorTemplate {
        unit: "°C".into(),
        location: "Hackerspace".into(),
        name: Value("Room Temperature".into()),
        description: Absent,
    }), "temperature_room".into());
    server.register_sensor(Box::new(PeopleNowPresentSensorTemplate {
        location: Value("Hackerspace".into()),
        name: Absent,
        description: Absent,
        names: Absent,
    }), "people_now_present".into());

    // Serve!
    server.serve().unwrap_or_else(|e| {
        println!("Could not start server: {:?}", e);
        exit(1);
    });
}
