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
