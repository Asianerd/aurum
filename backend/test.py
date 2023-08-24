from PyQt5.QtGui import *
from PyQt5.QtWidgets import QApplication

import pyqtgraph as pg

from pyqtgraph.Qt import QtCore

window = pg.plot()
window.hide()
window.setGeometry(100, 100, 800, 600)

window.addItem(pg.BarGraphItem(x=[10], y=[10], width=[10], height=[50]))
