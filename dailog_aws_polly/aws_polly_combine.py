# -*- coding: utf-8 -*-
"""
Created on Thu Jun  9 16:47:32 2022

@author: BemelmansRFJ
"""

# voeg alle mp3 bestanden in de werkdirectory samen tot 1 mp3

from pydub import AudioSegment
import glob
import os

# sorteer bestanden op tijd
MP3_FILES = sorted(glob.glob('output/*.mp3'), key=os.path.getmtime)

print(MP3_FILES)

# pauze tussen bestanden
silence = AudioSegment.silent(duration=500)
full_audio = AudioSegment.empty()

for n, mp3_file in enumerate(MP3_FILES):
    print(n, mp3_file)

    audio_segment = AudioSegment.from_mp3(mp3_file)

    full_audio += audio_segment + silence
    print('Merging ', n)

full_audio.export('combined/result.mp3', format='mp3')
print('\ndone!')