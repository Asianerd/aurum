import json
from flask import Flask
from flask_cors import CORS, cross_origin
app = Flask(__name__)
cors = CORS(app)
app.config['CORS_HEADERS'] = 'Content-Type'

@app.route('/')
def index():
    return "Can you understand me?"

@app.route('/all')
@cross_origin() # not sure if this decorator is needed
def other_stuff():
    with open('parsed_stocks.json', 'r') as file:
        data = json.load(file)
        data.sort(key=lambda x:x["sell"], reverse=True)
        return data[0:10]

app.run(debug=True, host='0.0.0.0')
