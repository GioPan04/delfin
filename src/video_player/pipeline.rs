use anyhow::Result;
use gst::{prelude::*, Element, ElementFactory, Pipeline};

pub fn create_pipeline(url: &str, sink: Box<Element>) -> Pipeline {
    let src = ElementFactory::make("uridecodebin")
        .property("uri", url)
        .build()
        .unwrap();

    let pipeline = Pipeline::default();
    pipeline.add(&src).expect("Error adding source to pipeline");
    let pipeline_weak = pipeline.downgrade();

    src.connect_pad_added(move |_, src_pad| {
        let pipeline = match pipeline_weak.upgrade() {
            Some(pipeline) => pipeline,
            None => return,
        };

        let is_video = src_pad.current_caps().and_then(|caps| {
            caps.structure(0).map(|s| {
                let name = s.name();
                name.starts_with("video/")
            })
        });
        let is_video = match is_video {
            None => {
                // TODO: handle error
                println!("Failed to get video from pad {}", src_pad.name());
                return;
            }
            Some(is_video) => is_video,
        };

        let insert_sink = |is_video| -> Result<()> {
            if is_video {
                let queue = ElementFactory::make("queue2").build()?;
                let valve = ElementFactory::make("valve").name("valve").build()?;
                let convert = ElementFactory::make("videoconvert").build()?;
                let els = &[&queue, &valve, &convert, &sink];
                pipeline.add_many(els)?;
                Element::link_many(els)?;
                for e in els {
                    e.sync_state_with_parent()?;
                }

                let sink_pad = queue.static_pad("sink").expect("queue has no sink pad");
                src_pad.link(&sink_pad)?;
            }

            Ok(())
        };

        if let Err(err) = insert_sink(is_video) {
            // TODO: handle error
            println!("Error inserting sink: {:#?}", err);
        };
    });

    pipeline
}
