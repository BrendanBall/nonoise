use ladspa::{Plugin, PluginDescriptor, Port, PortConnection};
use nnnoiseless_plugin::Denoise;
use std::default::Default;

struct Nnnoiseless {
    denoise: Box<Denoise>,
}

fn new_nnnoiseless(_: &PluginDescriptor, _sample_rate: u64) -> Box<dyn Plugin + Send> {
    Box::new(Nnnoiseless {
        denoise: Denoise::new(),
    })
}

impl Plugin for Nnnoiseless {
    fn run<'a>(&mut self, _sample_count: usize, ports: &[&'a PortConnection<'a>]) {
        let input = ports[0].unwrap_audio();
        let mut output = ports[1].unwrap_audio_mut();
        self.denoise.process(input, &mut output, input.len(), 0.95)
    }
}
#[repr(C)]
pub enum FFIOption<T> {
    None,
    Some(T),
}

#[no_mangle]
pub extern "C" fn get_ladspa_descriptor(index: u64) -> FFIOption<PluginDescriptor> {
    match index {
        0 => FFIOption::Some(PluginDescriptor {
            unique_id: 401,
            label: "nnnoiseless",
            properties: ladspa::PROP_NONE,
            name: "Nnnoiseless",
            maker: "Brendan",
            copyright: "None",
            ports: vec![
                Port {
                    name: "Audio In",
                    desc: ladspa::PortDescriptor::AudioInput,
                    ..Default::default()
                },
                Port {
                    name: "Audio Out",
                    desc: ladspa::PortDescriptor::AudioOutput,
                    ..Default::default()
                },
            ],
            new: new_nnnoiseless,
        }),
        _ => FFIOption::None,
    }
}
