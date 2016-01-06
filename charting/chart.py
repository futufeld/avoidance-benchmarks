from __future__ import division

import json
import os, sys

from matplotlib import rc
import matplotlib.patches as mpatches
import matplotlib.pyplot as plt
import numpy as np
import pylab

#
#
#

rc('text', usetex=True)
rc('font', family='serif')
rc('font', serif='Computer Modern')
rc('text.latex', preamble=r'\usepackage{amsmath}')

rc('axes', linewidth=1)

rc('xtick', direction='out')
rc('ytick', direction='out')

rc('xtick.major', size=8, width=1, pad=0)
rc('xtick.minor', size=4, width=1, pad=0)

rc('ytick.major', size=8, width=1, pad=10)
rc('ytick.minor', size=4, width=1, pad=10)

rc('xtick', labelsize=14)
rc('ytick', labelsize=14)

#
# Per-benchmark charting
#

def chart(filepath):
    """
    Generates a box plot of the benchmark data in the file with name filepath.
    """

    def compare_benchmarks(x, y):
        """
        Partitions benchmarks according to whether they involve possible
        collisions, then sorts by how many obstacles are involved. Used to
        sort benchmarks in preparation of charting.
        """
        if x[0]['significant'] > 0 and y[0]['significant'] == 0:
            return 1
        if x[0]['significant'] == 0 and y[0]['significant'] > 0:
            return -1
        return 1 if x[0]['total'] > y[0]['total'] else -1

    with open(filepath, 'r') as data_file:
        data_json = json.loads(data_file.read())

        # Collect data from json.
        benchmarks = []
        for benchmark in data_json:
            obstacles = benchmark['obstacles']
            run_times = benchmark['batch']['run_times']
            benchmarks += [ (obstacles, run_times) ]

            # Print proportion of outliers.
            q75, q25 = np.percentile(run_times, [75, 25])
            upper = q75 + 1.5 * (q75 - q25)
            lower = q25 - 1.5 * (q75 - q25)
            outliers = filter(lambda x: x > upper or x < lower, run_times)
            percentage_outliers = (len(outliers) / len(run_times)) * 100
            print "Outliers: {:2.2f}".format(percentage_outliers)

        benchmarks = sorted(benchmarks, compare_benchmarks)

        # Create chart.
        fig, ax = plt.subplots()
        plt.subplots_adjust(left=0.15, right=0.95, top=0.90, bottom=0.15)

        # Plot data.
        data = [ benchmark[1] for benchmark in benchmarks ]
        high = len(data)
        half = int(high / 2)
        positions = range(1,half+1) + range(half+2, high+2)
        
        medianprops = dict(linestyle='-', linewidth=1, color='k')
        params = dict( positions = positions
                     , patch_artist = True
                     , medianprops = medianprops
                     , showfliers=False )
        box = plt.boxplot(data, **params)

        # Configure box appearance.
        pylab.setp(box['caps'], color='k')
        pylab.setp(box['fliers'], color='none')
        pylab.setp(box['whiskers'], color='k', ls='-')

        # Set box colors.
        for patch in box['boxes']:
            patch.set_facecolor('w')
            patch.set_edgecolor('k')

        # Set box labels.
        labels = []
        for benchmark in benchmarks:
            labels += [ str(benchmark[0]['total']) ]
        labels.insert(high, '')
        labels.insert(half, '')
        labels.insert(0, '')

        ax.set_xticks(range(0, len(benchmarks) + 3))
        ax.set_xticklabels(labels)
        ax.set_xlabel(r'Number of obstacles', size=18, labelpad=20)
        ax.set_ylabel(r'Time ($\mu$\,s)', size=18, labelpad=10)

        # Configure chart appearance.
        ax.spines['right'].set_visible(False)
        ax.spines['top'].set_visible(False)

        ax.xaxis.set_ticks_position('none')
        ax.yaxis.set_ticks_position('left')

        # Indicate scenario type.
        params = dict(ls='dashed', fc='0.95', ec='none', zorder=-1)
        
        coords1 = (0.5, half - 0.5)
        ax.add_patch(mpatches.Rectangle(coords1, half, 100000, **params))
        
        coords2 = (half + 1.5, high - 0.5)
        ax.add_patch(mpatches.Rectangle(coords2, half, 100000, **params))

        params = dict(size=16, transform=ax.transAxes, ha='center', zorder=2)
        plt.text(0.25, 1.025, r'Insignificant', **params)
        plt.text(0.75, 1.025, r'Significant', **params)

#
# Utilities
#

def save(name, dpi=300):
    """Saves current plot to file in local './figures'."""
    filepath = os.path.join(os.getcwd(), 'figures', name)
    plt.savefig(filepath, dpi=dpi)

    ext = name[name.rindex('.')+1:]
    if ext == 'pdf':
        os.system('pdfcrop --margins 30 ' + filepath + ' ' + filepath + ' >/dev/null')
    elif ext == 'png':
        os.system('convert -trim ' + filepath + ' ' + filepath)
        os.system('convert -bordercolor White -border 50x50 ' + filepath + ' ' + filepath)

def present(save_chart=False, filename='', dpi=150):
    """If true, saves the figure, otherwise presents it in a matplotlib window."""
    if save_chart:
        if len(filename) == 0:
            raise Exception('mpl_utils.present: No filename supplied.')
        save(filename, dpi)
    else:
        plt.show()

#
# Entry point
#

if __name__ == '__main__':
    if len(sys.argv) > 1:
        if len(sys.argv) == 3:
            chart(sys.argv[1])
            present(True, sys.argv[2])
        else:
            present()
