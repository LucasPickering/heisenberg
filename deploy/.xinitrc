unclutter -idle 0 -root &
xrdb -merge ~/.Xresources
# After heisenberg exits, sleep indefinitely to show output. Prevents a boot loop
exec xterm -e 'sh -c "cd ~/heisenberg; ./heisenberg; sleep infinity"'
