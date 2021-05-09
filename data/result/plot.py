import numpy as np
import matplotlib.pyplot as plt
for i in np.arange(0,1.1,0.1):
    data=np.loadtxt("./"+str(i)[:3]+"_layer.txt")
    plt.title("sigma="+str(i)[:3])
    img=plt.contourf(data)
    plt.colorbar(img)
    plt.savefig("../../image/"+str(i)[:3]+"_sigma.jpg")
    plt.close()
