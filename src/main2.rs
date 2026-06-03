// TODO: ideal-ish main
fn main() {
    // includes background color
    let world = sample_scene();

    let camera = VideoCamera::new(vfov);
    // look_from, look_at, up, vfov);
    camera.set_aperture(23.0);
    camera.set_shutter_speed(1.0 / 200.0);
    camera.set_frame_rate(23.999);
    camera.set_initial_focus(1.4);

    camera.add_motion(new_location, new_orientation, time_start, time_end);
    camera.add_focus(new_dist, time_start, time_end);
    camera.add_motion_curve(
        some_curve_position,
        some_curve_orientation,
        time_start,
        time_end,
    );

    // dbg!(camera.total_frames());

    let renderer = MonteCarloRenderer(100, 10); // samples_per_pixel, max_depth
    renderer.add_prebake_step(Prebake::PhotonMap);

    let film = BasicFilm::new(1080.0); // Option<Filter> eventually
    // film.set_aspect_ratio(4.0/3.0);
    // contains aspect ratio and h_resolution
    camera.load_film(film); // consumes

    let image_writer = MultiFilePngWriter::new("./output", "frame_{frame_number}.png");

    camera.roll(renderer); // calls renderer.prebake_all()

    while camera.is_rolling() {
        // I think it makes sense to
        // borrow render config (reused acress frames), consume frame config (not needed after
        // captureframe), and borrow scene (reused across frames)
        //
        // calls camera.advance_frame, gets world state at current frame, calls
        // renderer.render
        let frame = camera.capture_frame(world);
        // Frame:
        //      pixels
        //      w (from film)
        //      h (from film)
        //      frame_number
        //      t
        //      camera settings (for debugging)
        image_writer.write(frame);
    }
}
