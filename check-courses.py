def clearConsole():
    import os
    command = 'clear'
    if os.name in ('nt', 'dos'):
        command = 'cls'
    os.system(command)

def categorize(tag):
    try:
        return tag.find('input')['value']
    except:
        return 'nicht möglich'

def check(sport, url, verbose=False):
    import requests
    from bs4 import BeautifulSoup

    requests = requests.get(url)
    unparsed = requests.text.encode("utf-8")
    soup = BeautifulSoup(unparsed, 'lxml')
    
    course_ids = list(map(lambda tag: tag.find('a')['id'], soup.findAll('td', class_='bs_sbuch')))
    course_availability = list(map(lambda tag: categorize(tag), soup.findAll('td', class_='bs_sbuch')))
    course_names = list(map(lambda tag: tag.string, soup.findAll('td', class_='bs_sdet')))
    course_days = list(map(lambda tag: tag.string, soup.findAll('td', class_='bs_stag')))
    course_times = list(map(lambda tag: tag.string, soup.findAll('td', class_='bs_szeit')))
    
    output = ''
    for i in range(0, len(course_names)):
        if not verbose:
            if course_availability[i] == 'buchen':
                output = output + sport + ': ' + course_names[i] + ' [' + course_ids[i] + ']' + '\n'
                output = output + course_days[i] + ' ' + course_times[i] + ' | ' + course_availability[i] + '\n'
                output = output + '\n'
        else:
            output = output + sport + ': ' + course_names[i] + ' [' + course_ids[i] + ']' + '\n'
            output = output + course_days[i] + ' ' + course_times[i] + ' | ' + course_availability[i] + '\n'
            output = output + '\n'
    print(output)
    if len(course_ids) < 1:
        print(requests)

if __name__ == "__main__":
    import time, sys
    vb = 'https://www2.usz.uni-halle.de/angebote/aktueller_zeitraum/_Volleyball.html'
    bm = 'https://www2.usz.uni-halle.de/angebote/aktueller_zeitraum/_Badminton.html'
    tn = 'https://www2.usz.uni-halle.de/angebote/aktueller_zeitraum/_Tennis.html'
    tp = 'https://www2.usz.uni-halle.de/angebote/aktueller_zeitraum/_Tennisplatz.html'
    tt = 'https://www2.usz.uni-halle.de/angebote/aktueller_zeitraum/_Tischtennis.html'
    yg = 'https://www2.usz.uni-halle.de/angebote/aktueller_zeitraum/_Yoga__Hatha_Yoga__bei_C__Brettschneider.html'
    pl = 'https://www2.usz.uni-halle.de/angebote/aktueller_zeitraum/_Pilates_-_Stretching_-_Feldenkrais.html'
    timer = 30
    verbose = False
    try:
        timer = int(sys.argv[1])
    except Exception as e:
        pass
    if 'verbose' in sys.argv:
        verbose = True

    while True:
        clearConsole()
        if 'vb' in sys.argv or 'volleyball' in sys.argv:
            check('Volleyball', vb, verbose)
        if 'bm' in sys.argv or 'badminton' in sys.argv:
            check('Badminton', bm, verbose)
        if 'tn' in sys.argv or 'tennis' in sys.argv:
            check('Tennis', tn, verbose)
        if 'tp' in sys.argv or 'tennisplatz' in sys.argv:
            check('Tennisplatz', tp, verbose)
        if 'tt' in sys.argv or 'tischtennis' in sys.argv:
            check('Tischtennis', tt, verbose)
        if 'yg' in sys.argv or 'yoga' in sys.argv:
            check('Yoga', yg, verbose)
        if 'pl' in sys.argv or 'pilates' in sys.argv:
            check('Pilates', pl, verbose)
        if len(sys.argv) <= 1:
            check('Volleyball', vb, verbose)
        print(time.ctime())
        time.sleep(timer)