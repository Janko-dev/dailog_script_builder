# -*- coding: utf-8 -*-
"""
Created on Thu Jun  9 16:47:32 2022

@author: BemelmansRFJ
"""

from boto3 import Session
from contextlib import closing
import os
import json
import config

session = Session(
    aws_access_key_id=config.api_key,                     
    aws_secret_access_key=config.api_secret,
    region_name='eu-central-1')
polly = session.client("polly")

script_json = "input/script.json"
name_dict = {
    "B": "Joey",
    "J": "Kimberly",
    "D": "Joanna",
}

# json-bestand: per text 1 mp3
with open(script_json, 'r') as f:
    script_dict = json.load(f)

i=1
# script_dict['script'] is een list van dictionaries
for sentence in script_dict['script']:

    stem = name_dict[sentence["name"]]
        
    response = polly.synthesize_speech(Text=sentence['text'],
                                        TextType="ssml",
                                        Engine="neural",
                                        OutputFormat="mp3",
                                        VoiceId=stem)    
        
        
    with closing(response["AudioStream"]) as stream:
        output = os.path.join("output", f"{i}{sentence['name']}.mp3")
        with open(output, "wb") as file:
            file.write(stream.read())
    i+=1
 