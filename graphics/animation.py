import matplotlib.pyplot as plt 
import numpy as np
import matplotlib.animation as animation
from matplotlib.colors import ListedColormap, BoundaryNorm
import pathlib



cmap = ListedColormap(["yellow", "orange", "red", "purple"])
bounds = [0, 1, 2, 3, 4]
norm = BoundaryNorm(bounds, cmap.N)

def yield_sandbox(path, size):
    with open(path,"r") as f:    
        for i,line in enumerate(f):
            if line.startswith("-") or line.startswith("_") or line == "\n":
                continue
            else:
                yield read_one_box(f,size)

def line_to_arr(line):
    return [int(i) for i in line]    

def read_one_box(f, size):
    size = 32
    box = np.zeros((size, size))
    for i in range(size-1):
        arr = line_to_arr(f.readline().strip())
        box[i,:] = arr
        
    return box

def animate(path, size):
    frames = yield_sandbox(path, size)

    fig, ax = plt.subplots()

    # get the first frame (required to initialize)
    first_frame = next(frames)
    im = ax.imshow(first_frame, cmap=cmap, norm=norm, animated=True)
    ax.set_xticks([])
    ax.set_yticks([])

    # update function for FuncAnimation
    def update(frame):
        im.set_array(frame)
        return (im,)

    ani = animation.FuncAnimation(
        fig,
        update,
        frames=frames,    # generator that yields matrices
        interval=20,     # ms between frames
        blit=True,
        save_count=2000
    )

    # If you want to save:
    ani.save("sandbox_evolution.mp4", fps=5, dpi=200)

def main():
    path = pathlib.Path("../sandbox.log")
    size = 32
    animate(path,size)
        
    
    

if __name__ == "__main__":
    main()