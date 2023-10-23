import numpy as np
import pandas as pd

PATH = './awesome/path.parquet'
df = pd.read_csv(PATH, dtypes={'col_1': 'float', 'col_2': 'str'})

df['time'] = pd.to_datetime(df['col_2'])
