
if [ ! -d "videos" ] 
then
    mkdir videos
fi

# ffmpeg -framerate 5 -start_number 0 -i out/image-%d.png -vframes 100 videos/out.mp4
ffmpeg -framerate 60 -i out/image-%d.png -vcodec libx264 -crf 25  -pix_fmt yuv420p videos/out.mp4