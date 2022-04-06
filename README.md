# usz-checker
checks if usz servers are online and which courses are available

## how to use:

example 1: displays all volleyball courses that can be booked, refreshes every 30 seconds
```
python check-courses.py
```

example 2: displays all tennis and badminton courses, refreshes every minute
```
python check-courses.py 60 verbose tennis badminton
```
- the first option determines the refresh delay in seconds, default is 30 seconds
- using the option ```verbose``` will display courses even if they are full or sign up is not yet available
- possible arguments for sports are ```volleyball```, ```tennis```, ```tennisplatz```, ```badminton```, ```tischtennis```, ```yoga```
- if no arguments are given volleyball will be checked