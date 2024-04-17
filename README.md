# file-to-luau
converts a file to a Roblox compatible luau script

# installing
Using [foreman](https://github.com/Roblox/foreman) add this tool. Replace the version with whatever the most recent version is in releases
```toml
[tools]
file-to-luau = {source="nightcycle/file-to-luau", version="1.0.0"}
```

# using
```sh
file-to-luau --input "path/to/input.ext" --out "path/to/output.luau"
```
## optional arguments
##### key
For table formats such as csv, tsv, and xlsx, you can choose to use a value from them as a key to a dictionary. Otherwise it will create it as an array of rows.
```
file-to-luau --input tests/input/infrastruct.xlsx --out tests/output/id-xlsx.luau --key Id
```
##### page
For an xlsx you can specify the specific page it will export, otherwise it will just choose the first one.
```
cargo run -- --input tests/input/infrastruct.xlsx --out tests/output/page-xlsx.luau --page City
```

# supports
## txt
```sh
file-to-luau --input tests/input/hello.txt --out tests/output/txt.luau
```
##### input
```txt
hello world
```

##### output
```lua
return `hello world`
```

## csv & tsv
```sh
file-to-luau --input tests/input/vehicle.csv --out tests/output/csv.luau
```
##### input
```csv
Id,Vehicle Compatibility,Name,Manufacturer,Model,Description,Max Speed,Max Speed Car Count,Fuel Cost Per Kilowatt,Air Pressure,Base Engine Weight,Base Car Weight,Max Car Cargo Weight,Car Cargo Volume,Car Passenger Limit,Friction Coefficient,Drag Coefficient,Lifespan,Construction Cost,Maintenance Cost
Rail_Callisto,Rail,Callisto,Lafayette Locomotive Works,No. 119,"The Callisto is classic American steam locomotive, using immense mechanical forces to slowly pull massive amounts of cargo across the nation.",50 kph,16,$0.03,1.00atm,50T,25T,100T,75m^3,0,0.00350,0.80,30yr,"$1,000K",$33K
Rail_Denver,Rail,Denver,Monarch,E7-F,The Denver is an iconic American streamliner locomotive known for providing decades of reliable service.,70 kph,30,$0.27,1.00atm,100T,25T,100T,75m^3,0,0.00350,0.95,40yr,"$2,000K",$50K
```
##### output
```lua
return {
	{
		["Air Pressure"] = "1.00atm",
		["Base Car Weight"] = "25T",
		["Base Engine Weight"] = "50T",
		["Car Cargo Volume"] = "75m^3",
		["Car Passenger Limit"] = 0,
		["Construction Cost"] = "$1,000K",
		["Description"] = "The Callisto is classic American steam locomotive, using immense mechanical forces to slowly pull massive amounts of cargo across the nation.",
		["Drag Coefficient"] = 0.8,
		["Friction Coefficient"] = 0.0035,
		["Fuel Cost Per Kilowatt"] = "$0.03",
		["Id"] = "Rail_Callisto",
		["Lifespan"] = "30yr",
		["Maintenance Cost"] = "$33K",
		["Manufacturer"] = "Lafayette Locomotive Works",
		["Max Car Cargo Weight"] = "100T",
		["Max Speed"] = "50 kph",
		["Max Speed Car Count"] = 16,
		["Model"] = "No. 119",
		["Name"] = "Callisto",
		["Vehicle Compatibility"] = "Rail",
	},
	{
		["Air Pressure"] = "1.00atm",
		["Base Car Weight"] = "25T",
		["Base Engine Weight"] = "100T",
		["Car Cargo Volume"] = "75m^3",
		["Car Passenger Limit"] = 0,
		["Construction Cost"] = "$2,000K",
		["Description"] = "The Denver is an iconic American streamliner locomotive known for providing decades of reliable service.",
		["Drag Coefficient"] = 0.95,
		["Friction Coefficient"] = 0.0035,
		["Fuel Cost Per Kilowatt"] = "$0.27",
		["Id"] = "Rail_Denver",
		["Lifespan"] = "40yr",
		["Maintenance Cost"] = "$50K",
		["Manufacturer"] = "Monarch",
		["Max Car Cargo Weight"] = "100T",
		["Max Speed"] = "70 kph",
		["Max Speed Car Count"] = 30,
		["Model"] = "E7-F",
		["Name"] = "Denver",
		["Vehicle Compatibility"] = "Rail",
	},
}
```

## json
```sh
file-to-luau --input tests/input/rojo.json --out tests/output/json.luau
```
##### input
```json
{
     "name": "test-project@0.1.0",
     "globIgnorePaths": [
          "**/package.json",
          "**/tsconfig.json"
     ],
	"num_test": [1, 2,3, 4],
	"bool_test": true,
     "tree": {
          "$className": "DataModel",
          "ReplicatedFirst": {
               "$className": "ReplicatedFirst",
               "First": {
                    "$className": "Folder",
                    "$path": "src/First",
                    "Boot": {
                         "$path": "out/First/Boot"
                    }
               },
			"Packages": {
                    "$className": "Folder",
                    "$path": "Packages"
               }
          },
          "ReplicatedStorage": {
               "$className": "ReplicatedStorage",
               "Packages": {
                    "$className": "Folder",
                    "$path": "Packages"
               },
               "Asphalt": {
                    "$className": "Folder",
                    "$path": "asphalt/ReplicatedStorage"
               },
               "Shared": {
                    "$className": "Folder",
                    "$path": "src/Shared",
                    "Asphalt": {
                         "$path": "out/Shared/Asphalt"
                    },
                    "PseudoEnum": {
                         "$path": "out/Shared/PseudoEnum.luau"
                    },
                    "Scene": {
                         "$path": "out/Shared/Scene.luau"
                    },
                    "Version": {
                         "$path": "out/Shared/Version.luau"
                    }
               },
               "Client": {
                    "$className": "Folder",
                    "$path": "src/Client",
                    "StyleGuide": {
                         "$path": "out/Client/StyleGuide"
                    }
               }
          },
          "MaterialService": {
               "$className": "MaterialService",
               "$path": "asphalt/MaterialService"
          },
          "SoundService": {
               "$className": "SoundService",
               "$path": "asphalt/SoundService"
          },
          "ServerScriptService": {
               "$className": "ServerScriptService",
               "Server": {
                    "$className": "Folder",
                    "$path": "src/Server",
                    "Boot": {
                         "$path": "out/Server/Boot"
                    }
               }
          },
          "StarterPlayer": {
               "$className": "StarterPlayer",
               "StarterPlayerScripts": {
                    "Boot": {
                         "$path": "out/Client/Boot"
                    }
               }
          }
     }
}
```
##### output
```lua
return {
	["bool_test"] = true,
	["globIgnorePaths"] = {
		"**/package.json",
		"**/tsconfig.json",
	},
	["name"] = "test-project@0.1.0",
	["num_test"] = {
		1,
		2,
		3,
		4,
	},
	["tree"] = {
		["$className"] = "DataModel",
		["MaterialService"] = {
			["$className"] = "MaterialService",
			["$path"] = "asphalt/MaterialService",
		},
		["ReplicatedFirst"] = {
			["$className"] = "ReplicatedFirst",
			["First"] = {
				["$className"] = "Folder",
				["$path"] = "src/First",
				["Boot"] = {
					["$path"] = "out/First/Boot",
				},
			},
			["Packages"] = {
				["$className"] = "Folder",
				["$path"] = "Packages",
			},
		},
		["ReplicatedStorage"] = {
			["$className"] = "ReplicatedStorage",
			["Asphalt"] = {
				["$className"] = "Folder",
				["$path"] = "asphalt/ReplicatedStorage",
			},
			["Client"] = {
				["$className"] = "Folder",
				["$path"] = "src/Client",
				["StyleGuide"] = {
					["$path"] = "out/Client/StyleGuide",
				},
			},
			["Packages"] = {
				["$className"] = "Folder",
				["$path"] = "Packages",
			},
			["Shared"] = {
				["$className"] = "Folder",
				["$path"] = "src/Shared",
				["Asphalt"] = {
					["$path"] = "out/Shared/Asphalt",
				},
				["PseudoEnum"] = {
					["$path"] = "out/Shared/PseudoEnum.luau",
				},
				["Scene"] = {
					["$path"] = "out/Shared/Scene.luau",
				},
				["Version"] = {
					["$path"] = "out/Shared/Version.luau",
				},
			},
		},
		["ServerScriptService"] = {
			["$className"] = "ServerScriptService",
			["Server"] = {
				["$className"] = "Folder",
				["$path"] = "src/Server",
				["Boot"] = {
					["$path"] = "out/Server/Boot",
				},
			},
		},
		["SoundService"] = {
			["$className"] = "SoundService",
			["$path"] = "asphalt/SoundService",
		},
		["StarterPlayer"] = {
			["$className"] = "StarterPlayer",
			["StarterPlayerScripts"] = {
				["Boot"] = {
					["$path"] = "out/Client/Boot",
				},
			},
		},
	},
}
```

## toml
```sh
file-to-luau --input tests/input/pseudo-enum.toml --out tests/output/toml.luau
```
##### input
```toml
build_path = "out/Shared/PseudoEnum.luau"
use_union_types_for_tree = false
use_union_types_for_export = true
date_test = 1979-05-27T00:32:00.999999-07:00

[enums]
AnalyticsPlatform = ["PC", "Mobile", "Console", "VR", "Unknown"]
AnalyticsMapEvent = ["Reset", "Fell", "Spawned", "Interval"]
AnalyticsMapArea = [
	"BrickPublicHousing",
	"ArtDecoBigOffice",
	"ArtDecoOldBusiness",
	"ArtDecoSuiteTower",
	"ModernistArtisticApartment",
	"ModernistFancyMediumApartment",
	"ModernistFancySmallApartment",
	"ModernistLuxuryComplex",
	"InternationalBillionaireHeights",
	"InternationalLuxuryTower",
	"InternationalOfficePurgatory",
	"InternationalTechTopia",
	"ColonialAntiqueStore",
	"ChesswickCreamery",
	"ColonialMuseum",
	"ColonialNotAPub",
	"ColonialOldHouse",
	"ColonialShop",
	"BrutalistAffordableCondo",
	"BrutalistBigWindowApartment",
	"BrutalistConcreteOffice",
	"BrutalistNiceCondo",
	"BrutalistShopApartments",
	"EnPizza",
	"BrickBookstore",
	"BrickStore",
	"TheQueensGrounds",
	"BrickFancyApartment",
	"BrickLocalBusinessHub",
	"CenterPark",
	"CornerTreePark",
	"Other",
]
```
##### output
```lua
return {
	build_path = "out/Shared/PseudoEnum.luau",
	date_test = DateTime.fromUniversalTime(1979, 5, 27, 0, 32, 0, 999999000 / 1000000),
	enums = {
		AnalyticsMapArea = {
			"BrickPublicHousing",
			"ArtDecoBigOffice",
			"ArtDecoOldBusiness",
			"ArtDecoSuiteTower",
			"ModernistArtisticApartment",
			"ModernistFancyMediumApartment",
			"ModernistFancySmallApartment",
			"ModernistLuxuryComplex",
			"InternationalBillionaireHeights",
			"InternationalLuxuryTower",
			"InternationalOfficePurgatory",
			"InternationalTechTopia",
			"ColonialAntiqueStore",
			"ChesswickCreamery",
			"ColonialMuseum",
			"ColonialNotAPub",
			"ColonialOldHouse",
			"ColonialShop",
			"BrutalistAffordableCondo",
			"BrutalistBigWindowApartment",
			"BrutalistConcreteOffice",
			"BrutalistNiceCondo",
			"BrutalistShopApartments",
			"EnPizza",
			"BrickBookstore",
			"BrickStore",
			"TheQueensGrounds",
			"BrickFancyApartment",
			"BrickLocalBusinessHub",
			"CenterPark",
			"CornerTreePark",
			"Other",
		},
		AnalyticsMapEvent = {
			"Reset",
			"Fell",
			"Spawned",
			"Interval",
		},
		AnalyticsPlatform = {
			"PC",
			"Mobile",
			"Console",
			"VR",
			"Unknown",
		},
	},
	use_union_types_for_export = true,
	use_union_types_for_tree = false,
}
```

## xlsx
```sh
file-to-luau --input tests/input/infrastruct.xlsx --out tests/output/page-xlsx.luau --key Id
```
##### input
```
binary nonsense
```
##### output
```lua
return {
	["HighSpeedRail_Pacifique"] = {
		["Air Pressure"] = 1,
		["Base Car Weight"] = 25,
		["Base Engine Weight"] = 75,
		["Car Cargo Volume"] = 75,
		["Car Passenger Limit"] = 0,
		["Construction Cost"] = 2500,
		["Description"] = "The Pacifique is a world famous European HSR allowing for quick transportation between countries at affordable prices.",
		["Drag Coefficient"] = 0.3,
		["Friction Coefficient"] = 0.0035,
		["Fuel Cost Per Kilowatt"] = 0.2699914015,
		["Id"] = "HighSpeedRail_Pacifique",
		["Lifespan"] = 25,
		["Maintenance Cost"] = 100,
		["Manufacturer"] = "SNCR",
		["Max Car Cargo Weight"] = 100,
		["Max Speed"] = 160,
		["Max Speed Car Count"] = 12,
		["Model"] = "T-1988",
		["Name"] = "Pacifique",
		["Vehicle Compatibility"] = "HighSpeedRail",
	},
	["HyperloopRail_Gigapod"] = {
		["Air Pressure"] = 0.2,
		["Base Car Weight"] = 25,
		["Base Engine Weight"] = 50,
		["Car Cargo Volume"] = 75,
		["Car Passenger Limit"] = 0,
		["Construction Cost"] = 10000,
		["Description"] = "The Gigapod is about speed above everything else, including fiscal responsibility.",
		["Drag Coefficient"] = 0.1,
		["Friction Coefficient"] = 0.00035,
		["Fuel Cost Per Kilowatt"] = 0.03374892519,
		["Id"] = "HyperloopRail_Gigapod",
		["Lifespan"] = 15,
		["Maintenance Cost"] = 666.6666667,
		["Manufacturer"] = "Sturgeon",
		["Max Car Cargo Weight"] = 100,
		["Max Speed"] = 1200,
		["Max Speed Car Count"] = 12,
		["Model"] = "TT20-F",
		["Name"] = "Gigapod",
		["Vehicle Compatibility"] = "HyperloopRail",
	},
	["MaglevRail_Magnus"] = {
		["Air Pressure"] = 1,
		["Base Car Weight"] = 25,
		["Base Engine Weight"] = 50,
		["Car Cargo Volume"] = 75,
		["Car Passenger Limit"] = 0,
		["Construction Cost"] = 5000,
		["Description"] = "The Magnus is a record-breaking Japanese maglev that serves as the backbone for fast transit between densely populated cities. The 'Magnus' branding is an English localization to help market the technology overseas.",
		["Drag Coefficient"] = 0.2,
		["Friction Coefficient"] = 0.00035,
		["Fuel Cost Per Kilowatt"] = 0.06749785039,
		["Id"] = "MaglevRail_Magnus",
		["Lifespan"] = 25,
		["Maintenance Cost"] = 200,
		["Manufacturer"] = "Magurebu",
		["Max Car Cargo Weight"] = 100,
		["Max Speed"] = 600,
		["Max Speed Car Count"] = 12,
		["Model"] = "TK46",
		["Name"] = "Magnus",
		["Vehicle Compatibility"] = "MaglevRail",
	},
	["Monorail_Gulf"] = {
		["Air Pressure"] = 1,
		["Base Car Weight"] = 25,
		["Base Engine Weight"] = 75,
		["Car Cargo Volume"] = 75,
		["Car Passenger Limit"] = 0,
		["Construction Cost"] = 5000,
		["Description"] = "The Gulf is a retro-futuristic vision for urban transit, however it is also very capable in dealing with high incline mountainous regions due to how it grips the track.",
		["Drag Coefficient"] = 0.5,
		["Friction Coefficient"] = 0.004375,
		["Fuel Cost Per Kilowatt"] = 0.2024935512,
		["Id"] = "Monorail_Gulf",
		["Lifespan"] = 30,
		["Maintenance Cost"] = 166.6666667,
		["Manufacturer"] = "Walt Motors",
		["Max Car Cargo Weight"] = 100,
		["Max Speed"] = 180,
		["Max Speed Car Count"] = 8,
		["Model"] = "D71",
		["Name"] = "Gulf",
		["Vehicle Compatibility"] = "Monorail",
	},
	["Rail_Callisto"] = {
		["Air Pressure"] = 1,
		["Base Car Weight"] = 25,
		["Base Engine Weight"] = 50,
		["Car Cargo Volume"] = 75,
		["Car Passenger Limit"] = 0,
		["Construction Cost"] = 1000,
		["Description"] = "The Callisto is classic American steam locomotive, using immense mechanical forces to slowly pull massive amounts of cargo across the nation.",
		["Drag Coefficient"] = 0.8,
		["Friction Coefficient"] = 0.0035,
		["Fuel Cost Per Kilowatt"] = 0.03439380911,
		["Id"] = "Rail_Callisto",
		["Lifespan"] = 30,
		["Maintenance Cost"] = 33.33333333,
		["Manufacturer"] = "Lafayette Locomotive Works",
		["Max Car Cargo Weight"] = 100,
		["Max Speed"] = 50,
		["Max Speed Car Count"] = 16,
		["Model"] = "No. 119",
		["Name"] = "Callisto",
		["Vehicle Compatibility"] = "Rail",
	},
	["Rail_Denver"] = {
		["Air Pressure"] = 1,
		["Base Car Weight"] = 25,
		["Base Engine Weight"] = 100,
		["Car Cargo Volume"] = 75,
		["Car Passenger Limit"] = 0,
		["Construction Cost"] = 2000,
		["Description"] = "The Denver is an iconic American streamliner locomotive known for providing decades of reliable service.",
		["Drag Coefficient"] = 0.95,
		["Friction Coefficient"] = 0.0035,
		["Fuel Cost Per Kilowatt"] = 0.2699914015,
		["Id"] = "Rail_Denver",
		["Lifespan"] = 40,
		["Maintenance Cost"] = 50,
		["Manufacturer"] = "Monarch",
		["Max Car Cargo Weight"] = 100,
		["Max Speed"] = 70,
		["Max Speed Car Count"] = 30,
		["Model"] = "E7-F",
		["Name"] = "Denver",
		["Vehicle Compatibility"] = "Rail",
	},
}
```

## yaml
##### input
```yaml
build:
  client_boot_script_path: out/Client/Analytics.client.luau
  midas_py_module_out_path: src/data/download
  server_boot_script_path: out/Server/Analytics.server.luau
  shared_state_tree_path: out/Shared/MidasTree.luau
encoding_marker: '~'
playfab:
  download_start_date: "1970-1-01 00:00"
  download_window: 30
  user_limit: 10000000
products:
  developer_products: {}
  gamepasses: {}
recorder:
  id:
    group_id: 67890
    place_id: 12345
  interval: 60
  out_path: src/data/record
state_tree:
  Duration: integer
  Id:
    Place: string
    Session: string
    User: string
  Index:
    Event: integer
    Total: integer
  IsStudio: boolean
  Version:
    Build: integer
    Major: integer
    Minor: integer
    Patch: integer
templates:
  character: false
  chat: false
  client_performance: false
  demographics: false
  exit: false
  group: {
    "Nightcycle": 4181328,
  }
  join: false
  market: false
  player: false
  population: false
  server_performance: false
version:
  hotfix: 0
  major: 1
  minor: 1
  patch: 5
```
##### output
```lua
return {
	build = {
		client_boot_script_path = "out/Client/Analytics.client.luau",
		midas_py_module_out_path = "src/data/download",
		server_boot_script_path = "out/Server/Analytics.server.luau",
		shared_state_tree_path = "out/Shared/MidasTree.luau",
	},
	encoding_marker = "~",
	playfab = {
		download_start_date = DateTime.fromUniversalTime(1970, 1, 1, 0, 0, 0, 0 / 1000000),
		download_window = 30,
		user_limit = 10000000,
	},
	products = {
		developer_products = {},
		gamepasses = {},
	},
	recorder = {
		id = {
			group_id = 67890,
			place_id = 12345,
		},
		interval = 60,
		out_path = "src/data/record",
	},
	state_tree = {
		Duration = "integer",
		Id = {
			Place = "string",
			Session = "string",
			User = "string",
		},
		Index = {
			Event = "integer",
			Total = "integer",
		},
		IsStudio = "boolean",
		Version = {
			Build = "integer",
			Major = "integer",
			Minor = "integer",
			Patch = "integer",
		},
	},
	templates = {
		character = false,
		chat = false,
		client_performance = false,
		demographics = false,
		exit = false,
		group = {
			Nightcycle = 4181328,
		},
		join = false,
		market = false,
		player = false,
		population = false,
		server_performance = false,
	},
	version = {
		hotfix = 0,
		major = 1,
		minor = 1,
		patch = 5,
	},
}
```
