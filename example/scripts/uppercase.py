import pandas as pd

df = pd.read_csv('top-songs-transit.csv')



filtered = df.query('name == "Good 4 U Olivia Rodrigo"')

print(filtered.head()) 

filtered.to_csv("top-songs-transit.csv")