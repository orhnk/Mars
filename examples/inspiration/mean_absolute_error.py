import numpy as np
def mean_absolute_error(act, pred):
    diff = pred - act
    abs_diff = np.absolute(diff)
    mean_diff = abs_diff.mean()
    return mean_diff

act = np.array([1.1,2,1.7])
pred = np.array([1,1.7,1.5])
print(mean_absolute_error(act,pred))
