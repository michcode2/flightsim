import json
import numpy as np
import sys

def velocity_magnitude(components):
    return np.power([k for k in components.values()], 2).sum() ** 0.5

def climb_angle(components):
    return np.arctan(components['z']/horizontal_velocity(components))

def horizontal_velocity(components): 
    return velocity_magnitude({'x': components['x'], 'y': components['y']})

states = json.load(open("pitch_up.json"))['data']
positions = [s['position'] for s in states]
pointings = [s['pointing_global'] for s in states]
velocities = [s['velocity'] for s in states]
positions = [s['position'] for s in states]
accelerations = [s['acceleration'] for s in states]

if max(np.gradient([velocity_magnitude(v) for v in velocities])) < 0:
    sys.exit(0)
else:
    sys.exit(255)