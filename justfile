convert:
  ffmpeg -framerate 24 -pattern_type glob -i './output/frame_*.png' -c:v libx264 -pix_fmt yuv420p output.mp4 && just clean

clean:
  rm ./output/frame_*.png
