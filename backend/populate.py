import random
import json

# #region compound names
raw = """
1	Acetic acid	60.052 g/mol	CH3COOH
2	Hydrochloric acid	36.458 g/mol	HCl
3	Sulfuric acid	‎98.072 g/mol	H2SO4
4	Acetate	59.044 g/mol	CH3COO–
5	Ammonia	17.031 g/mol	NH3
6	Nitric acid	63.012 g/mol	HNO3
7	Phosphoric acid	97.994 g/mol	H3PO4
8	Sodium phosphate	119.976 g/mol	Na3PO4
9	Calcium carbonate	100.086 g/mol	CaCO3
10	Ammonium sulfate	132.134 g/mol	(NH4)2SO4
11	Carbonic acid	62.024 g/mol	H2CO3
12	Sodium bicarbonate	84.0066 g/mol	NaHCO3
13	Sodium hydroxide	39.997 g/mol	NaOH
14	Calcium hydroxide	74.092 g/mol	Ca(OH)2
15	Ethanol	46.069 g/mol	C2H5OH
16	Hydrobromic acid	80.912 g/mol	HBr
17	Nitrous acid	‎47.013 g/mol	HNO2
18	Potassium hydroxide	56.11 g/mol	KOH
19	Silver nitrate	169.872 g/mol	AgNO3
20	Sodium carbonate	105.988 g/mol	Na2CO3
21	Sodium chloride	58.44 g/mol	NaCl
22	Cellulose	162.1406 g/mol	(C6H10O5)n
23	Magnesium hydroxide	58.319 g/mol	Mg(OH)2
24	Methane	16.043 g/mol	CH4
25	Nitrogen dioxide	30.006 g/mol	NO2
26	Sodium nitrate	84.994 g/mol	NaNO3
27	Sulfurous acid	82.073 g/mol	H2SO3
28	Aluminium sulfate	342.15 g/mol	Al2(SO4)3
29	Aluminum oxide	101.96 g/mol	Al2O3
30	Ammonium nitrate	80.043 g/mol	NH4NO3
31	Ammonium phosphate	132.056 g/mol	(NH4)3PO4
32	Barium hydroxide	171.341 g/mol	Ba(OH)2
33	Carbon tetrachloride	153.811 g/mol	CCl4
34	Citric acid	192.123 g/mol	C6H8O7
35	Hydrocyanic acid	‎27.026 g/mol	HCN
36	Salicylic Acid	138.121 g/mol	C7H6O3
37	Hydroiodic acid	127.91 g/mol	HI
38	Hypochlorous acid	52.457 g/mol	HClO
39	Iron iii oxide	159.687 g/mol	Fe2O3
40	Magnesium phosphate	262.855 g/mol	Mg3(PO4)2
41	Sodium acetate	82.0343 g/mol	C2H3NaO2
42	Sodium sulfate	‎142.036 g/mol	Na2SO4
43	Sucrose	342.2965 g/mol	C12H22O11
44	Potassium nitrate	101.102 g/mol	KNO3
45	Ammonium bicarbonate	96.086 g/mol	NH4HCO3
46	Ammonium chloride	53.489 g/mol	NH4Cl
47	Ammonium hydroxide	35.046 g/mol	NH4OH
48	Calcium nitrate	164.088 g/mol	Ca(NO3)2
49	Calcium oxide	56.0774 g/mol	CaO
50	Carbon monoxide	28.01 g/mol	CO
51	Chlorine gas	70.9 g/mol	Cl2
53	Phenol	94.11 g/mol	C6H6O
54	Hydrogen peroxide	34.0147 g/mol	H2O2
55	Hydroxide	17.007 g/mol	OH–
56	Magnesium chloride	95.211 g/mol	MgCl2
57	Potassium chloride	74.5513 g/mol	KCl
58	Potassium iodide	166.0028 g/mol	KI
59	Sulfur dioxide	64.066 g/mol	SO2
60	Glycerin	92.09 g/mol	C3H8O3
61	Barium nitrate	261.337 g/mol	Ba(NO3)2
62	Calcium acetate	158.17 g/mol	C4H6O4Ca
63	Iron oxide	159.69 g/mol	Fe2O3
64	Potassium carbonate	138.205 g/mol	K2CO3
65	Silver chloride	‎143.318 g/mol	AgCl
66	Sodium iodide	149.894 g/mol	NaI
67	Sodium oxide	61.9789 g/mol	Na2O
68	Sodium sulfide	78.0452 g/mol	Na2S
69	Zinc nitrate	189.388 g/mol	Zn(NO3)2
70	Phenolphthalein	318.32 g/mol	C20H14O4
71	Magnesium nitrate	148.313 g/mol	Mg(NO3)2
72	Silicon dioxide	60.083 g/mol	SiO2
73	Acetone	58.08 g/mol	C3H6O
74	Hydroquinone	110.11 g/mol	C6H6O2
75	Pyridine	79.1 g/mol	C5H5N
76	Ammonium acetate	77.083 g/mol	C2H3O2NH4
77	Xylene	106.16 g/mol	C8H10
78	Barium sulfate	233.38 g/mol	BaSO4
79	Benzene	78.11 g/mol	C6H6
80	Bicarbonate	61.016 g/mol	CHO3–
81	Chromate	15.992 g/mol	CrO42-
82	Methyl Ethyl Ketone	72.107 g/mol	C4H8O
83	Cyanide	26.02 g/mol	CN−
84	Trichloroacetic acid	163.38 g/mol	C2HCl3O2
85	Magnesium sulfate	120.361 g/mol	MgSO4
86	Methanol	32.04 g/mol	CH3OH
87	Oxygen	Atomic mass 15.999 g/mol	O
88	Methylene blue	319.85 g/mol	C16H18ClN3S
89	Sodium sulfite	126.043 g/mol	Na2SO3
90	Sulfur trioxide	80.057 g/mol	SO3
91	Aluminum phosphate	121.951 g/mol	AlPO4
92	Stearic acid	284.484 g/mol	C18H36O2
93	Dinitrogen monoxide	44.013 g/mol	N2O
94	Titanium dioxide	233.38 g/mol	TiO2
95	Acetonitrile	41.053 g/mol	C2H3N
96	Oxalic acid	90.03 g/mol	H2C2O4
97	Potassium dichromate	294.185 g/mol	K2Cr2O7
98	Sodium bromide	102.894 g/mol	NaBr
99	Sodium hypochlorite	74.439 g/mol	NaClO
100	Zinc acetate	183.48 g/mol	Zn(CH3COO)2(H2O)2
101	Zinc chloride	136.286 g/mol	ZnCl2
102	Zinc hydroxide	99.424 g/mol	Zn(OH)2
103	Magnesium carbonate	84.313 g/mol	MgCO3
104	Potassium chlorate	122.545 g/mol	KClO3
105	Hydrazine	32.0452 g/mol	N2H4
106	Ascorbic acid	176.12 g/mol	C6H8O6
107	Benzoic acid	122.12 g/mol	C7H6O2
108	Resorcinol	110.1 g/mol	C6H6O2
109	Chlorine	‎70.9 g/mol	Cl2
110	Maleic acid	116.072 g/mol	C4H4O4
111	Sodium metabisulfite	190.107 g/mol	Na2S2O5
112	Acetamide	59.068 g/mol	C2H5NO
113	Sodium silicate	122.062 g/mol	(Na2O)x·SiO2
114	Nitrite	46.005 g/mol	NO2−
115	Phosphate	94.9714 g/mol	PO43-
116	Dichloromethane	84.93 g/mol	CH2Cl2
117	Carbon Disulfide	76.13 g/mol	CS2
118	Potassium chromate	194.189 g/mol	CrK2O4
119	Zinc sulfate	161.436 g/mol	ZnSO4
120	Iodine	Atomic mass 126.90 g/mol	I
121	Tannic acid	1701.19 g/mol	C76H52O46
122	Aluminum	26.982 g/mol	Al
123	Perchloric acid	100.46 g/mol	HClO4
124	Hypochlorite	51.449 g/mol	ClO–
125	Potassium Bromide	119.002 g/mol	KBr
126	Chromic acid	118.01 g/mol	H2CrO4
127	Dihydrogen monoxide	18.01528 g/mol	OH2
128	Methyl acetate	74.079 g/mol	C3H6O2
129	Dimethyl sulfoxide	78.13 g/mol	C2H6OS
130	Hexane	86.18 g/mol	C6H14
131	Eugenol	164.2 g/mol	C10H12O2
132	Manganese dioxide	86.9368 g/mol	MnO2
133	Lactic acid	90.078 g/mol	C3H6O3
134	Sodium potassium tartrate	282.1 g/mol	C4H4O6KNa·4H2O
135	Hexamine	140.186 g/mol	C6H12N4
136	Lithium hydroxide	23.95 g/mol	LiOH
137	Phosphorus pentachloride	208.24 g/mol	PCl5
138	Potassium oxide	94.2 g/mol	K2O
139	Monopotassium phosphate	136.084 g/mol	KH2PO4
140	Silver acetate	166.91 g/mol	AgC2H3O2
141	Sodium citrate	258.06 g/mol	Na3C6H5O7
142	Sodium fluoride	41.98817 g/mol	NaF
143	Sodium nitrite	68.9953 g/mol	NaNO2
144	Sulfate ion	96.06 g/mol	SO42−
145	Barium carbonate	197.34 g/mol	BaCO3
146	Calcium iodide	293.887 g/mol	CaI2
147	Hydrogen sulfate	97.064 g/mol	HSO4–
148	Lithium oxide	29.88 g/mol	Li2O
149	Dimethylglyoxime	116.12 g/mol	C4H8N2O2
150	Potassium Permanganate	158.034 g/mol	KMnO4
151	Silver phosphate	418.58 g/mol	Ag3PO4
152	Ammonium bromide	97.943 g/mol	NH4Br
153	Calcium phosphate	310.18 g/mol	Ca3(PO4)2
154	Dichromate	294.185 g/mol	K2Cr2O7
155	Aluminum sulfide	150.158 g/mol	Al2S3
156	Ammonium carbonate	96.086 g/mol	(NH4)2CO3
157	Barium chloride	208.23 g/mol	BaCl2
158	Nitrogen monoxide	30.006 g/mol	NO
159	Fructose	180.16 g/mol	C6H12O6
160	Magnesium iodide	278.1139 g/mol	MgI2
161	Magnesium sulfide	56.38 g/mol	MgS
162	Ozone	48 g/mol	O3
163	Potassium cyanide	65.12 g/mol	KCN
164	Silver oxide	231.735 g/mol	Ag2O
165	Sodium chromate	161.97 g/mol	Na2CrO4
166	Sodium peroxide	77.98 g/mol	Na2O2
167	Toluene	92.14 g/mol	C7H8
168	Zinc carbonate	125.388 g/mol	ZnCO3
169	Zinc phosphate	386.11 g/mol	Zn3(PO4)2
170	Zinc sulfide	97.474 g/mol	ZnS
171	Para dichlorobenzene	147.01 g/mol	C6H4Cl2
172	Boric acid	61.83 g/mol	H3BO3
173	Oxalate	‎88.018 g/mol	C2O42−
174	Potassium bicarbonate	100.114 g/mol	KHCO3
175	Potassium hypochlorite	90.55 g/mol	KClO
176	Potassium nitrite	85.103 g/mol	KNO2
177	Bromothymol Blue	624.384 g/mol	C27H28Br2O5S
178	Ammonium iodide	144.94 g/mol	NH4I
179	Ammonium nitrite	64.06 g/mol	NH4NO2
180	Ammonium oxide	52.0763 g/mol	(NH4)2O
181	Argon gas	39.948 g/mol	Ar
182	Barium bromide	297.14 g/mol	BaBr2
183	Barium iodide	391.136 g/mol	BaI2
184	Bromate	127.901 g/mol	BrO3–
185	Dinitrogen trioxide	76.01 g/mol	N2O3
186	Ethylene glycol	62.07 g/mol	C2H6O2
187	Nickel sulfate	154.75 g/mol	NiSO4
188	Helium	atomic mass ‎4.002602 u	He
189	Iodide	253.809 g/mol	I–
190	Lead ii acetate	‎325.29 g/mol	Pb(C2H3O2)2
191	Lithium chloride	42.394 g/mol	LiCl
192	Phosphate ion	94.9714 g/mol	PO43-
193	Potassium fluoride	58.0967 g/mol	KF
194	Potassium sulfite	158.26 g/mol	K2SO3
195	Silver carbonate	275.7453 g/mol	Ag2CO3
196	Sodium cyanide	49.0072 g/mol	NaCN
197	Sodium nitride	82.976 g/mol	Na3N
198	Strontium chloride	158.52 g/mol	SrCl2
199	Strontium nitrate	211.628 g/mol	Sr(NO3)2
200	Urea	‎60.056 g/mol	CH4N2O
201	Bleach	‎74.439 g/mol	NaClO
202	Lithium bromide	86.844 g/mol	LiBr
203	Aluminum fluoride	83.9767 g/mol	AlF3
204	Barium fluoride	175.34 g/mol	BaF2
205	Butanoic acid	88.11 g/mol	C4H8O2
206	Calcium hydride	42.094 g/mol	CaH2
207	Copper ii carbonate	123.55 g/mol	CuCO3
208	Fluorine	18.998403 u	F
209	Lithium phosphate	115.79 g/mol	Li3PO4
210	Glycerol	92.09382 g/mol	C3H8O3
211	Hypobromous acid	96.911 g/mol	HBrO
212	Hypoiodous acid	143.89 g/mol	HIO
213	Lead iodide	461.01 g/mol	PbI2
214	Lithium iodide	133.844 g/mol	LiI
215	Magnesium oxide	40.3044 g/mol	MgO
216	Urethane	89.09 g/mol	 C3H7NO2
217	Nickel nitrate	182.703 g/mol	Ni(NO3)2
218	Sodium dichromate	261.97 g/mol	Na2Cr2O7
219	Tartaric acid	150.087 g/mol	C4H6O6
220	Zinc iodide	319.22 g/mol	ZnI2
221	Bromine	54.9380 g/mol	Br
222	Aluminum bromide	266.69 g/mol	AlBr3
223	Sodium Percarbonate	157.01 g/mol	C2H6Na4O12
224	Nickel acetate	178.797 g/mol	C4H6O4Ni
225	Sodium Thiosulfate	158.11 g/mol	Na2S2O3
226	Acetaldehyde	44.05 g/mol	C2H4O
227	Copper sulfate	159.609 g/mol	CuSO4
228	Mannitol	182.172 g/mol	C6H14O6
229	Calcium Chloride	110.98 g/mol	CaCl2
230	Monosodium Glutamate	169.111 g/mol	C5H8NO4Na
231	Polystyrene	104.1 g/mol	(C8H8)n
232	Calcium Carbide	64.099 g/mol	CaC2
233	Tetrachloroethylene	165.83 g/mol	C2Cl4
234	Sodium Chlorate	106.44 g/mol	NaClO3
235	Potassium Iodate	214.001 g/mol	KIO3
236	Lead Acetate	325.29 g/mol	Pb(C2H3O2)2
237	Potassium Thiocyanate	97.181 g/mol	KSCN
238	Butane	58.12 g/mol	C4H10
239	Maltose	342.3 g/mol	C12H22O11
240	Polyurethane Foam	548.589 g/mol	C27H36N2O10
241	Formaldehyde	30.031 g/mol	CH2O
242	Formic Acid	46.03 g/mol	HCOOH
243	Sulfur Hexafluoride	146.06 g/mol	SF6
244	Phosphorus Trichloride	137.33 g/mol	PCl3
245	Ethane	30.07 g/mol	C2H6
246	Dinitrogen Pentoxide	30.07 g/mol	N2O5
247	Phosphorous Acid	82 g/mol	H3PO3
248	Potassium Ferrocyanide	368.35 g/mol	K4Fe(CN)6
249	Xenon Difluoride	169.29 g/mol	XeF2
250	Diatomic Bromine	159.808 g/mol	Br2
251	Phenyl	77.106 g/mol	C6H5
252	Phosphorus Triiodide	411.6872 g/mol	PI3
253	Peroxydisulfuric Acid	194.14 g/mol	H2S2O8
254	Dipotassium Phosphate	174.2 g/mol	K2HPO4
255	Aluminium hydroxide	78.00 g/mol	Al(OH)3
256	Ammonium persulfate	228.18 g/mol	(NH4)2S2O8
257	Sodium borate	201.22 g/mol	Na2[B4O5(OH)4]·8H2O
258	Chloroacetic acid	94.49 g/mol	C2H3O2Cl
259	Potassium acetate	98.142 g/mol	CH3CO2K
260	Barium oxide	153.326 g/mol	BaO
261	Copper(I) Oxide	143.09 g/mol	Cu2O
262	Copper Hydroxide	97.561 g/mol	Cu(OH)2
263	Tin Oxide	97.561 g/mol	SnO2
264	Chlorine Trifluoride	92.448 g/mol	ClF3
265	Ethylene	28.054 g/mol	C2H4
266	Acetylene	26.038 g/mol	C2H2
267	Chromic Oxide	151.9904 g/mol	Cr2O3
268	Sodium bisulfate	120.06 g/mol	NaHSO4
269	Copper (II) chloride	134.45 g/mol	CuCl2
270	Mercuric chloride	271.52 g/mol	HgCl2
271	Tin (II) chloride	189.60 g/mol	SnCl2
272	Propane	44.097 g/mol	C3H8
273	Lead (IV) oxide	239.1988 g/mol	PbO2
"""
#endregion

raw = """
3Com Corp
3M Company
A.G. Edwards Inc.
Abbott Laboratories
Abercrombie & Fitch Co.
ABM Industries Incorporated
Ace Hardware Corporation
ACT Manufacturing Inc.
Acterna Corp.
Adams Resources & Energy, Inc.
ADC Telecommunications, Inc.
Adelphia Communications Corporation
Administaff, Inc.
Adobe Systems Incorporated
Adolph Coors Company
Advance Auto Parts, Inc.
Advanced Micro Devices, Inc.
AdvancePCS, Inc.
Advantica Restaurant Group, Inc.
The AES Corporation
Aetna Inc.
Affiliated Computer Services, Inc.
AFLAC Incorporated
AGCO Corporation
Agilent Technologies, Inc.
Agway Inc.
Apartment Investment and Management Company
Air Products and Chemicals, Inc.
Airborne, Inc.
Airgas, Inc.
AK Steel Holding Corporation
Alaska Air Group, Inc.
Alberto-Culver Company
Albertson's, Inc.
Alcoa Inc.
Alleghany Corporation
Allegheny Energy, Inc.
Allegheny Technologies Incorporated
Allergan, Inc.
ALLETE, Inc.
Alliant Energy Corporation
Allied Waste Industries, Inc.
Allmerica Financial Corporation
The Allstate Corporation
ALLTEL Corporation
The Alpine Group, Inc.
Amazon.com, Inc.
AMC Entertainment Inc.
American Power Conversion Corporation
Amerada Hess Corporation
AMERCO
Ameren Corporation
America West Holdings Corporation
American Axle & Manufacturing Holdings, Inc.
American Eagle Outfitters, Inc.
American Electric Power Company, Inc.
American Express Company
American Financial Group, Inc.
American Greetings Corporation
American International Group, Inc.
American Standard Companies Inc.
American Water Works Company, Inc.
AmerisourceBergen Corporation
Ames Department Stores, Inc.
Amgen Inc.
Amkor Technology, Inc.
AMR Corporation
AmSouth Bancorp.
Amtran, Inc.
Anadarko Petroleum Corporation
Analog Devices, Inc.
Anheuser-Busch Companies, Inc.
Anixter International Inc.
AnnTaylor Inc.
Anthem, Inc.
AOL Time Warner Inc.
Aon Corporation
Apache Corporation
Apple Computer, Inc.
Applera Corporation
Applied Industrial Technologies, Inc.
Applied Materials, Inc.
Aquila, Inc.
ARAMARK Corporation
Arch Coal, Inc.
Archer Daniels Midland Company
Arkansas Best Corporation
Armstrong Holdings, Inc.
Arrow Electronics, Inc.
ArvinMeritor, Inc.
Ashland Inc.
Astoria Financial Corporation
AT&T Corp.
Atmel Corporation
Atmos Energy Corporation
Audiovox Corporation
Autoliv, Inc.
Automatic Data Processing, Inc.
AutoNation, Inc.
AutoZone, Inc.
Avaya Inc.
Avery Dennison Corporation
Avista Corporation
Avnet, Inc.
Avon Products, Inc.
Baker Hughes Incorporated
Ball Corporation
Bank of America Corporation
The Bank of New York Company, Inc.
Bank One Corporation
Banknorth Group, Inc.
Banta Corporation
Barnes & Noble, Inc.
Bausch & Lomb Incorporated
Baxter International Inc.
BB&T Corporation
The Bear Stearns Companies Inc.
Beazer Homes USA, Inc.
Beckman Coulter, Inc.
Becton, Dickinson and Company
Bed Bath & Beyond Inc.
Belk, Inc.
Bell Microproducts Inc.
BellSouth Corporation
Belo Corp.
Bemis Company, Inc.
Benchmark Electronics, Inc.
Berkshire Hathaway Inc.
Best Buy Co., Inc.
Bethlehem Steel Corporation
Beverly Enterprises, Inc.
Big Lots, Inc.
BJ Services Company
BJ's Wholesale Club, Inc.
The Black & Decker Corporation
Black Hills Corporation
BMC Software, Inc.
The Boeing Company
Boise Cascade Corporation
Borders Group, Inc.
BorgWarner Inc.
Boston Scientific Corporation
Bowater Incorporated
Briggs & Stratton Corporation
Brightpoint, Inc.
Brinker International, Inc.
Bristol-Myers Squibb Company
Broadwing, Inc.
Brown Shoe Company, Inc.
Brown-Forman Corporation
Brunswick Corporation
Budget Group, Inc.
Burlington Coat Factory Warehouse Corporation
Burlington Industries, Inc.
Burlington Northern Santa Fe Corporation
Burlington Resources Inc.
C. H. Robinson Worldwide Inc.
Cablevision Systems Corp
Cabot Corp
Cadence Design Systems, Inc.
Calpine Corp.
Campbell Soup Co.
Capital One Financial Corp.
Cardinal Health Inc.
Caremark Rx Inc.
Carlisle Cos. Inc.
Carpenter Technology Corp.
Casey's General Stores Inc.
Caterpillar Inc.
CBRL Group Inc.
CDI Corp.
CDW Computer Centers Inc.
CellStar Corp.
Cendant Corp
Cenex Harvest States Cooperatives
Centex Corp.
CenturyTel Inc.
Ceridian Corp.
CH2M Hill Cos. Ltd.
Champion Enterprises Inc.
Charles Schwab Corp.
Charming Shoppes Inc.
Charter Communications Inc.
Charter One Financial Inc.
ChevronTexaco Corp.
Chiquita Brands International Inc.
Chubb Corp
Ciena Corp.
Cigna Corp
Cincinnati Financial Corp.
Cinergy Corp.
Cintas Corp.
Circuit City Stores Inc.
Cisco Systems Inc.
Citigroup, Inc
Citizens Communications Co.
CKE Restaurants Inc.
Clear Channel Communications Inc.
The Clorox Co.
CMGI Inc.
CMS Energy Corp.
CNF Inc.
Coca-Cola Co.
Coca-Cola Enterprises Inc.
Colgate-Palmolive Co.
Collins & Aikman Corp.
Comcast Corp.
Comdisco Inc.
Comerica Inc.
Comfort Systems USA Inc.
Commercial Metals Co.
Community Health Systems Inc.
Compass Bancshares Inc
Computer Associates International Inc.
Computer Sciences Corp.
Compuware Corp.
Comverse Technology Inc.
ConAgra Foods Inc.
Concord EFS Inc.
Conectiv, Inc
Conoco Inc
Conseco Inc.
Consolidated Freightways Corp.
Consolidated Edison Inc.
Constellation Brands Inc.
Constellation Emergy Group Inc.
Continental Airlines Inc.
Convergys Corp.
Cooper Cameron Corp.
Cooper Industries Ltd.
Cooper Tire & Rubber Co.
Corn Products International Inc.
Corning Inc.
Costco Wholesale Corp.
Countrywide Credit Industries Inc.
Coventry Health Care Inc.
Cox Communications Inc.
Crane Co.
Crompton Corp.
Crown Cork & Seal Co. Inc.
CSK Auto Corp.
CSX Corp.
Cummins Inc.
CVS Corp.
Cytec Industries Inc.
D&K Healthcare Resources, Inc.
D.R. Horton Inc.
Dana Corporation
Danaher Corporation
Darden Restaurants Inc.
DaVita Inc.
Dean Foods Company
Deere & Company
Del Monte Foods Co
Dell Computer Corporation
Delphi Corp.
Delta Air Lines Inc.
Deluxe Corporation
Devon Energy Corporation
Di Giorgio Corporation
Dial Corporation
Diebold Incorporated
Dillard's Inc.
DIMON Incorporated
Dole Food Company, Inc.
Dollar General Corporation
Dollar Tree Stores, Inc.
Dominion Resources, Inc.
Domino's Pizza LLC
Dover Corporation, Inc.
Dow Chemical Company
Dow Jones & Company, Inc.
DPL Inc.
DQE Inc.
Dreyer's Grand Ice Cream, Inc.
DST Systems, Inc.
DTE Energy Co.
E.I. Du Pont de Nemours and Company
Duke Energy Corp
Dun & Bradstreet Inc.
DURA Automotive Systems Inc.
DynCorp
Dynegy Inc.
E*Trade Group, Inc.
E.W. Scripps Company
Earthlink, Inc.
Eastman Chemical Company
Eastman Kodak Company
Eaton Corporation
Echostar Communications Corporation
Ecolab Inc.
Edison International
EGL Inc.
El Paso Corporation
Electronic Arts Inc.
Electronic Data Systems Corp.
Eli Lilly and Company
EMC Corporation
Emcor Group Inc.
Emerson Electric Co.
Encompass Services Corporation
Energizer Holdings Inc.
Energy East Corporation
Engelhard Corporation
Enron Corp.
Entergy Corporation
Enterprise Products Partners L.P.
EOG Resources, Inc.
Equifax Inc.
Equitable Resources Inc.
Equity Office Properties Trust
Equity Residential Properties Trust
Estee Lauder Companies Inc.
Exelon Corporation
Exide Technologies
Expeditors International of Washington Inc.
Express Scripts Inc.
ExxonMobil Corporation
Fairchild Semiconductor International Inc.
Family Dollar Stores Inc.
Farmland Industries Inc.
Federal Mogul Corp.
Federated Department Stores Inc.
Federal Express Corp.
Felcor Lodging Trust Inc.
Ferro Corp.
Fidelity National Financial Inc.
Fifth Third Bancorp
First American Financial Corp.
First Data Corp.
First National of Nebraska Inc.
First Tennessee National Corp.
FirstEnergy Corp.
Fiserv Inc.
Fisher Scientific International Inc.
FleetBoston Financial Co.
Fleetwood Enterprises Inc.
Fleming Companies Inc.
Flowers Foods Inc.
Flowserv Corp
Fluor Corp
FMC Corp
Foamex International Inc
Foot Locker Inc
Footstar Inc.
Ford Motor Co
Forest Laboratories Inc.
Fortune Brands Inc.
Foster Wheeler Ltd.
FPL Group Inc.
Franklin Resources Inc.
Freeport McMoran Copper & Gold Inc.
Frontier Oil Corp
Furniture Brands International Inc.
Gannett Co., Inc.
Gap Inc.
Gateway Inc.
GATX Corporation
Gemstar-TV Guide International Inc.
GenCorp Inc.
General Cable Corporation
General Dynamics Corporation
General Electric Company
General Mills Inc
General Motors Corporation
Genesis Health Ventures Inc.
Gentek Inc.
Gentiva Health Services Inc.
Genuine Parts Company
Genuity Inc.
Genzyme Corporation
Georgia Gulf Corporation
Georgia-Pacific Corporation
Gillette Company
Gold Kist Inc.
Golden State Bancorp Inc.
Golden West Financial Corporation
Goldman Sachs Group Inc.
Goodrich Corporation
The Goodyear Tire & Rubber Company
Granite Construction Incorporated
Graybar Electric Company Inc.
Great Lakes Chemical Corporation
Great Plains Energy Inc.
GreenPoint Financial Corp.
Greif Bros. Corporation
Grey Global Group Inc.
Group 1 Automotive Inc.
Guidant Corporation
H&R Block Inc.
H.B. Fuller Company
H.J. Heinz Company
Halliburton Co.
Harley-Davidson Inc.
Harman International Industries Inc.
Harrah's Entertainment Inc.
Harris Corp.
Harsco Corp.
Hartford Financial Services Group Inc.
Hasbro Inc.
Hawaiian Electric Industries Inc.
HCA Inc.
Health Management Associates Inc.
Health Net Inc.
Healthsouth Corp
Henry Schein Inc.
Hercules Inc.
Herman Miller Inc.
Hershey Foods Corp.
Hewlett-Packard Company
Hibernia Corp.
Hillenbrand Industries Inc.
Hilton Hotels Corp.
Hollywood Entertainment Corp.
Home Depot Inc.
Hon Industries Inc.
Honeywell International Inc.
Hormel Foods Corp.
Host Marriott Corp.
Household International Corp.
Hovnanian Enterprises Inc.
Hub Group Inc.
Hubbell Inc.
Hughes Supply Inc.
Humana Inc.
Huntington Bancshares Inc.
Idacorp Inc.
IDT Corporation
IKON Office Solutions Inc.
Illinois Tool Works Inc.
IMC Global Inc.
Imperial Sugar Company
IMS Health Inc.
Ingles Market Inc
Ingram Micro Inc.
Insight Enterprises Inc.
Integrated Electrical Services Inc.
Intel Corporation
International Paper Co.
Interpublic Group of Companies Inc.
Interstate Bakeries Corporation
International Business Machines Corp.
International Flavors & Fragrances Inc.
International Multifoods Corporation
Intuit Inc.
IT Group Inc.
ITT Industries Inc.
Ivax Corp.
J.B. Hunt Transport Services Inc.
J.C. Penny Co.
J.P. Morgan Chase & Co.
Jabil Circuit Inc.
Jack In The Box Inc.
Jacobs Engineering Group Inc.
JDS Uniphase Corp.
Jefferson-Pilot Co.
John Hancock Financial Services Inc.
Johnson & Johnson
Johnson Controls Inc.
Jones Apparel Group Inc.
KB Home
Kellogg Company
Kellwood Company
Kelly Services Inc.
Kemet Corp.
Kennametal Inc.
Kerr-McGee Corporation
KeyCorp
KeySpan Corp.
Kimball International Inc.
Kimberly-Clark Corporation
Kindred Healthcare Inc.
KLA-Tencor Corporation
K-Mart Corp.
Knight-Ridder Inc.
Kohl's Corp.
KPMG Consulting Inc.
Kroger Co.
L-3 Communications Holdings Inc.
Laboratory Corporation of America Holdings
Lam Research Corporation
LandAmerica Financial Group Inc.
Lands' End Inc.
Landstar System Inc.
La-Z-Boy Inc.
Lear Corporation
Legg Mason Inc.
Leggett & Platt Inc.
Lehman Brothers Holdings Inc.
Lennar Corporation
Lennox International Inc.
Level 3 Communications Inc.
Levi Strauss & Co.
Lexmark International Inc.
Limited Inc.
Lincoln National Corporation
Linens 'n Things Inc.
Lithia Motors Inc.
Liz Claiborne Inc.
Lockheed Martin Corporation
Loews Corporation
Longs Drug Stores Corporation
Louisiana-Pacific Corporation
Lowe's Companies Inc.
LSI Logic Corporation
The LTV Corporation
The Lubrizol Corporation
Lucent Technologies Inc.
Lyondell Chemical Company
M & T Bank Corporation
Magellan Health Services Inc.
Mail-Well Inc.
Mandalay Resort Group
Manor Care Inc.
Manpower Inc.
Marathon Oil Corporation
Mariner Health Care Inc.
Markel Corporation
Marriott International Inc.
Marsh & McLennan Companies Inc.
Marsh Supermarkets Inc.
Marshall & Ilsley Corporation
Martin Marietta Materials Inc.
Masco Corporation
Massey Energy Company
MasTec Inc.
Mattel Inc.
Maxim Integrated Products Inc.
Maxtor Corporation
Maxxam Inc.
The May Department Stores Company
Maytag Corporation
MBNA Corporation
McCormick & Company Incorporated
McDonald's Corporation
The McGraw-Hill Companies Inc.
McKesson Corporation
McLeodUSA Incorporated
M.D.C. Holdings Inc.
MDU Resources Group Inc.
MeadWestvaco Corporation
Medtronic Inc.
Mellon Financial Corporation
The Men's Wearhouse Inc.
Merck & Co., Inc.
Mercury General Corporation
Merrill Lynch & Co. Inc.
Metaldyne Corporation
Metals USA Inc.
MetLife Inc.
Metris Companies Inc
MGIC Investment Corporation
MGM Mirage
Michaels Stores Inc.
Micron Technology Inc.
Microsoft Corporation
Milacron Inc.
Millennium Chemicals Inc.
Mirant Corporation
Mohawk Industries Inc.
Molex Incorporated
The MONY Group Inc.
Morgan Stanley Dean Witter & Co.
Motorola Inc.
MPS Group Inc.
Murphy Oil Corporation
Nabors Industries Inc
Nacco Industries Inc
Nash Finch Company
National City Corp.
National Commerce Financial Corporation
National Fuel Gas Company
National Oilwell Inc
National Rural Utilities Cooperative Finance Corporation
National Semiconductor Corporation
National Service Industries Inc
Navistar International Corporation
NCR Corporation
The Neiman Marcus Group Inc.
New Jersey Resources Corporation
New York Times Company
Newell Rubbermaid Inc
Newmont Mining Corporation
Nextel Communications Inc
Nicor Inc
Nike Inc
NiSource Inc
Noble Energy Inc
Nordstrom Inc
Norfolk Southern Corporation
Nortek Inc
North Fork Bancorporation Inc
Northeast Utilities System
Northern Trust Corporation
Northrop Grumman Corporation
NorthWestern Corporation
Novellus Systems Inc
NSTAR
NTL Incorporated
Nucor Corp
Nvidia Corp
NVR Inc
Northwest Airlines Corp
Occidental Petroleum Corp
Ocean Energy Inc
Office Depot Inc.
OfficeMax Inc
OGE Energy Corp
Oglethorpe Power Corp.
Ohio Casualty Corp.
Old Republic International Corp.
Olin Corp.
OM Group Inc
Omnicare Inc
Omnicom Group
On Semiconductor Corp
ONEOK Inc
Oracle Corp
Oshkosh Truck Corp
Outback Steakhouse Inc.
Owens & Minor Inc.
Owens Corning
Owens-Illinois Inc
Oxford Health Plans Inc
Paccar Inc
PacifiCare Health Systems Inc
Packaging Corp. of America
Pactiv Corp
Pall Corp
Pantry Inc
Park Place Entertainment Corp
Parker Hannifin Corp.
Pathmark Stores Inc.
Paychex Inc
Payless Shoesource Inc
Penn Traffic Co.
Pennzoil-Quaker State Company
Pentair Inc
Peoples Energy Corp.
PeopleSoft Inc
Pep Boys Manny, Moe & Jack
Potomac Electric Power Co.
Pepsi Bottling Group Inc.
PepsiAmericas Inc.
PepsiCo Inc.
Performance Food Group Co.
Perini Corp
PerkinElmer Inc
Perot Systems Corp
Petco Animal Supplies Inc.
Peter Kiewit Sons', Inc.
PETsMART Inc
Pfizer Inc
Pacific Gas & Electric Corp.
Pharmacia Corp
Phar Mor Inc.
Phelps Dodge Corp.
Philip Morris Companies Inc.
Phillips Petroleum Co
Phillips Van Heusen Corp.
Phoenix Companies Inc
Pier 1 Imports Inc.
Pilgrim's Pride Corporation
Pinnacle West Capital Corp
Pioneer-Standard Electronics Inc.
Pitney Bowes Inc.
Pittston Brinks Group
Plains All American Pipeline LP
PNC Financial Services Group Inc.
PNM Resources Inc
Polaris Industries Inc.
Polo Ralph Lauren Corp
PolyOne Corp
Popular Inc
Potlatch Corp
PPG Industries Inc
PPL Corp
Praxair Inc
Precision Castparts Corp
Premcor Inc.
Pride International Inc
Primedia Inc
Principal Financial Group Inc.
Procter & Gamble Co.
Pro-Fac Cooperative Inc.
Progress Energy Inc
Progressive Corporation
Protective Life Corp
Provident Financial Group
Providian Financial Corp.
Prudential Financial Inc.
PSS World Medical Inc
Public Service Enterprise Group Inc.
Publix Super Markets Inc.
Puget Energy Inc.
Pulte Homes Inc
Qualcomm Inc
Quanta Services Inc.
Quantum Corp
Quest Diagnostics Inc.
Questar Corp
Quintiles Transnational
Qwest Communications Intl Inc
R.J. Reynolds Tobacco Company
R.R. Donnelley & Sons Company
Radio Shack Corporation
Raymond James Financial Inc.
Raytheon Company
Reader's Digest Association Inc.
Reebok International Ltd.
Regions Financial Corp.
Regis Corporation
Reliance Steel & Aluminum Co.
Reliant Energy Inc.
Rent A Center Inc
Republic Services Inc
Revlon Inc
RGS Energy Group Inc
Rite Aid Corp
Riverwood Holding Inc.
RoadwayCorp
Robert Half International Inc.
Rock-Tenn Co
Rockwell Automation Inc
Rockwell Collins Inc
Rohm & Haas Co.
Ross Stores Inc
RPM Inc.
Ruddick Corp
Ryder System Inc
Ryerson Tull Inc
Ryland Group Inc.
Sabre Holdings Corp
Safeco Corp
Safeguard Scientifics Inc.
Safeway Inc
Saks Inc
Sanmina-SCI Inc
Sara Lee Corp
SBC Communications Inc
Scana Corp.
Schering-Plough Corp
Scholastic Corp
SCI Systems Onc.
Science Applications Intl. Inc.
Scientific-Atlanta Inc
Scotts Company
Seaboard Corp
Sealed Air Corp
Sears Roebuck & Co
Sempra Energy
Sequa Corp
Service Corp. International
ServiceMaster Co
Shaw Group Inc
Sherwin-Williams Company
Shopko Stores Inc
Siebel Systems Inc
Sierra Health Services Inc
Sierra Pacific Resources
Silgan Holdings Inc.
Silicon Graphics Inc
Simon Property Group Inc
SLM Corporation
Smith International Inc
Smithfield Foods Inc
Smurfit-Stone Container Corp
Snap-On Inc
Solectron Corp
Solutia Inc
Sonic Automotive Inc.
Sonoco Products Co.
Southern Company
Southern Union Company
SouthTrust Corp.
Southwest Airlines Co
Southwest Gas Corp
Sovereign Bancorp Inc.
Spartan Stores Inc
Spherion Corp
Sports Authority Inc
Sprint Corp.
SPX Corp
St. Jude Medical Inc
St. Paul Cos.
Staff Leasing Inc.
StanCorp Financial Group Inc
Standard Pacific Corp.
Stanley Works
Staples Inc
Starbucks Corp
Starwood Hotels & Resorts Worldwide Inc
State Street Corp.
Stater Bros. Holdings Inc.
Steelcase Inc
Stein Mart Inc
Stewart & Stevenson Services Inc
Stewart Information Services Corp
Stilwell Financial Inc
Storage Technology Corporation
Stryker Corp
Sun Healthcare Group Inc.
Sun Microsystems Inc.
SunGard Data Systems Inc.
Sunoco Inc.
SunTrust Banks Inc
Supervalu Inc
Swift Transportation, Co., Inc
Symbol Technologies Inc
Synovus Financial Corp.
Sysco Corp
Systemax Inc.
Target Corp.
Tech Data Corporation
TECO Energy Inc
Tecumseh Products Company
Tektronix Inc
Teleflex Incorporated
Telephone & Data Systems Inc
Tellabs Inc.
Temple-Inland Inc
Tenet Healthcare Corporation
Tenneco Automotive Inc.
Teradyne Inc
Terex Corp
Tesoro Petroleum Corp.
Texas Industries Inc.
Texas Instruments Incorporated
Textron Inc
Thermo Electron Corporation
Thomas & Betts Corporation
Tiffany & Co
Timken Company
TJX Companies Inc
TMP Worldwide Inc
Toll Brothers Inc
Torchmark Corporation
Toro Company
Tower Automotive Inc.
Toys 'R' Us Inc
Trans World Entertainment Corp.
TransMontaigne Inc
Transocean Inc
TravelCenters of America Inc.
Triad Hospitals Inc
Tribune Company
Trigon Healthcare Inc.
Trinity Industries Inc
Trump Hotels & Casino Resorts Inc.
TruServ Corporation
TRW Inc
TXU Corp
Tyson Foods Inc
U.S. Bancorp
U.S. Industries Inc.
UAL Corporation
UGI Corporation
Unified Western Grocers Inc
Union Pacific Corporation
Union Planters Corp
Unisource Energy Corp
Unisys Corporation
United Auto Group Inc
United Defense Industries Inc.
United Parcel Service Inc
United Rentals Inc
United Stationers Inc
United Technologies Corporation
UnitedHealth Group Incorporated
Unitrin Inc
Universal Corporation
Universal Forest Products Inc
Universal Health Services Inc
Unocal Corporation
Unova Inc
UnumProvident Corporation
URS Corporation
US Airways Group Inc
US Oncology Inc
USA Interactive
USFreighways Corporation
USG Corporation
UST Inc
Valero Energy Corporation
Valspar Corporation
Value City Department Stores Inc
Varco International Inc
Vectren Corporation
Veritas Software Corporation
Verizon Communications Inc
VF Corporation
Viacom Inc
Viad Corp
Viasystems Group Inc
Vishay Intertechnology Inc
Visteon Corporation
Volt Information Sciences Inc
Vulcan Materials Company
W.R. Berkley Corporation
W.R. Grace & Co
W.W. Grainger Inc
Wachovia Corporation
Wakenhut Corporation
Walgreen Co
Wallace Computer Services Inc
Wal-Mart Stores Inc
Walt Disney Co
Walter Industries Inc
Washington Mutual Inc
Washington Post Co.
Waste Management Inc
Watsco Inc
Weatherford International Inc
Weis Markets Inc.
Wellpoint Health Networks Inc
Wells Fargo & Company
Wendy's International Inc
Werner Enterprises Inc
WESCO International Inc
Western Digital Inc
Western Gas Resources Inc
WestPoint Stevens Inc
Weyerhauser Company
WGL Holdings Inc
Whirlpool Corporation
Whole Foods Market Inc
Willamette Industries Inc.
Williams Companies Inc
Williams Sonoma Inc
Winn Dixie Stores Inc
Wisconsin Energy Corporation
Wm Wrigley Jr Company
World Fuel Services Corporation
WorldCom Inc
Worthington Industries Inc
WPS Resources Corporation
Wyeth
Wyndham International Inc
Xcel Energy Inc
Xerox Corp
Xilinx Inc
XO Communications Inc
Yellow Corporation
York International Corp
Yum Brands Inc.
Zale Corporation
Zions Bancorporation
"""

s = []
for x in raw.strip().split('\n'):
    s.append(x)

random.shuffle(s)

data = []
fidelity = 1_000_000_000

for x in s[0:300]:
    data.append({
        "id":len(data),
        "name":x,
        "record":[
            [
                0,
                0
                ],
            [
                random.randint(int(0.5 * fidelity), int(40 * fidelity)) / fidelity,
                0
                ]
            ],
        "angle":90000,
        "fluctuation":1000 + (random.randint(-fidelity, fidelity) / fidelity) * 1000,
        "buy_difference": 1 + random.randint(int(0.01 * fidelity), int(0.05 * fidelity)) / fidelity
        })

with open('stocks.json', 'w') as file:
    json.dump(data, file, indent=4)
