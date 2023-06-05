import matplotlib.pyplot as plt
import csv

alphas = [0, 0.25, 0.5, 0.75, 0.85, 1]
data = {a: [] for a in alphas}

if __name__ == "__main__":
    with open('L6/z14.csv') as csvfile:
        reader = csv.reader(csvfile, delimiter=',')
        for a,row in zip(alphas,reader):
            for item in row:
                data[a].append(float(item))

    for a in alphas:
        plt.plot(range(1,26), data[a][:25], label="alpha = {}".format(a))
    plt.legend()
    plt.title("Norm of difference of state from stationary state")
    plt.xlabel("Step")
    plt.ylabel("Norm")
    plt.savefig("L6/graph.png", dpi=300)
    plt.close()

