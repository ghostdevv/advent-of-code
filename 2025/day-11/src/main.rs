use std::collections::HashMap;

const INPUT: &str = include_str!("./input.txt");

const SEEN_FFT: i32 = 1 << 0;
const SEEN_DAC: i32 = 1 << 1;

enum Connection<'a> {
    Device(&'a str),
    Out,
}

impl<'a> From<&'a str> for Connection<'a> {
    fn from(value: &'a str) -> Self {
        if value == "out" {
            Connection::Out
        } else {
            Connection::Device(value)
        }
    }
}

struct Device<'a> {
    name: &'a str,
    connections: Vec<Connection<'a>>,
}

impl<'a> Device<'a> {
    fn count_routes(
        &self,
        devices: &HashMap<&'a str, Device<'a>>,
        cache: &mut HashMap<(&'a str, Option<i32>), i64>,
        seen: Option<i32>,
    ) -> i64 {
        if let Some(count) = cache.get(&(self.name, seen)) {
            return *count;
        }

        let mut count = 0;

        for connection in &self.connections {
            match connection {
                Connection::Device(name) => {
                    let device = devices
                        .get(name)
                        .unwrap_or_else(|| panic!("failed to get device {name}"));

                    let mut seen = seen.clone();

                    if let Some(s) = seen {
                        match *name {
                            "fft" => seen = Some(s | SEEN_FFT),
                            "dac" => seen = Some(s | SEEN_DAC),
                            _ => {}
                        }
                    }

                    count += device.count_routes(devices, cache, seen);
                }
                Connection::Out => match seen {
                    Some(s) => {
                        if s & SEEN_FFT != 0 && s & SEEN_DAC != 0 {
                            count += 1;
                        }
                    }
                    None => {
                        count += 1;
                    }
                },
            }
        }

        cache.insert((self.name, seen), count);

        count
    }
}

fn main() {
    let devices = INPUT
        .trim()
        .lines()
        .map(|line| {
            let (name, rest) = line
                .split_once(":")
                .unwrap_or_else(|| panic!("failed to split line: \"{line}\""));

            let connections = rest
                .trim()
                .split(" ")
                .map(|str| Connection::from(str))
                .collect::<Vec<_>>();

            if connections.len() == 0 {
                panic!("device has zero connections: {name}")
            }

            Device { name, connections }
        })
        .fold(HashMap::new(), |mut map, device| {
            map.insert(device.name, device);
            map
        });

    let mut cache = HashMap::new();

    let part_one = devices
        .get("you")
        .expect("Failed to get you")
        .count_routes(&devices, &mut cache, None);

    println!("Part One: {}", part_one);

    let part_two =
        devices
            .get("svr")
            .expect("Failed to get svr")
            .count_routes(&devices, &mut cache, Some(0));

    println!("Part Two: {:?}", part_two);
}
