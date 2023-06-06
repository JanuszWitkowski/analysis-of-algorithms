#!/usr/bin/python3
import matplotlib.pyplot as plt
import csv

# alphas = [0, 0.25, 0.5, 0.75, 0.85, 1]
# alphas = [0.25, 0.5, 0.75, 0.85, 1]
alphas = [0.75, 0.85, 1]
data = {alpha: [] for alpha in alphas}

if __name__ == "__main__":
    # with open('ex14_data.csv') as csvfile:
    # with open('ex14_data_non-zero.csv') as csvfile:
    with open('ex14_data_check0.85.csv') as csvfile:
        reader = csv.reader(csvfile, delimiter=';')
        for a,row in zip(alphas,reader):
            for item in row:
                data[a].append(float(item))

    plt.figure(figsize=(16, 8))
    for a in alphas:
        plt.plot(range(1,26), data[a][:25], label="alfa = {}".format(a))
    plt.legend()
    # plt.title("Zbieżność rozkładów stacjonarnych w zależności od alfy")
    # plt.title("Zbieżność rozkładów stacjonarnych w zależności od alfy (bez zera)")
    plt.title("Zbieżność rozkładów stacjonarnych w zależności od alfy bliskiej 0.85")
    plt.xlabel("Krok [1-25]")
    plt.ylabel("Różnica od właściwego rozkładu")
    # plt.savefig("ex14_plot.png", dpi=300)
    # plt.savefig("ex14_plot_non-zero.png", dpi=300)
    plt.savefig("ex14_plot_check0.85.png", dpi=300)
    plt.close()

