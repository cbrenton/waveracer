convert:
  ffmpeg -framerate 24 -pattern_type glob -i './output/frame_*.png' -c:v libx264 -pix_fmt yuv420p output.mp4

clean:
  rm ./output/frame_*.png

run_and_convert:
  cargo run --release && just convert
