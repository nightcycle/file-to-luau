return {
	build_path = "out/Shared/PseudoEnum.luau",
	date_test = DateTime.fromUniversalTime(1979, 5, 27, 0, 32, 0, 999999000 / 1000000),
	enums = {
		AnalyticsAIEvent = {
			"VoteOnMove",
			"GameError",
			"GameComplete",
			"GameStart",
		},
		AnalyticsChessGameEvent = {
			"EnterGame",
			"ExitGame",
		},
		AnalyticsCorrespondenceEvent = {
			"RegisterMatch",
			"LoadMatch",
			"MakeMove",
			"CompleteMatch",
		},
		AnalyticsEconomyEvent = {
			"BuyFood",
			"BuySkin",
			"UnlockSkin",
			"AIMatchReward",
			"TournamentReward",
			"BuyCurrency",
			"Allowance",
			"BuyApartment",
		},
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
		AnalyticsProductType = {
			"Gamepass",
			"Currency",
		},
		AnalyticsSessionEvent = {
			"Join",
			"Quit",
		},
		AnalyticsSocialEvent = {
			"Emote",
			"Chat",
		},
		AnalyticsTournamentEvent = {
			"TournamentStart",
			"TournamentComplete",
		},
		ChessAIEmote = {
			"Chess",
			"Trophy",
			"Angry",
			"Happy",
			"Skull",
			"Error",
			"Cry",
			"Grimace",
			"Robot",
			"Expressionless",
			"Nerd",
			"Sunglasses",
		},
		ChessAIStage = {
			"SelectPiece",
			"SelectMove",
			"AIMove",
			"GameLost",
			"GameWon",
			"Error",
		},
		ChessAIStageResolution = {
			"Complete",
			"Timeout",
			"Error",
		},
		ChessDisplayType = {
			"Mesh",
			"Gui",
			"Hybrid",
			"Letter",
		},
		ChessMatchType = {
			"Normal",
			"Tournament",
			"ServerVsBot",
		},
		ChessOutcome = {
			"White",
			"Black",
			"Stalemate",
		},
		ChessPerspectiveType = {
			"POV",
			"TopDown",
			"Side",
			"Isometric",
		},
		ContrastStandardType = {
			"Default",
			"LargeText",
			"Incidental",
			"Logotype",
		},
		GuiAlignmentType = {
			"Center",
			"Left",
			"Right",
			"Top",
			"Bottom",
			"TopLeft",
			"TopRight",
			"BottomLeft",
			"BottomRight",
		},
		GuiCategoryType = {
			"Background",
			"Panel",
			"Card",
			"Frame",
			"Item",
			"Button",
			"Label",
			"Bar",
			"Toggle",
		},
		GuiColorPalette = {
			"Primary1",
			"Primary2",
			"Primary3",
			"Primary4",
			"Primary5",
			"Primary6",
			"Secondary1",
			"Secondary2",
			"Secondary3",
			"Secondary4",
			"Secondary5",
			"Secondary6",
			"Tertiary1",
			"Tertiary2",
			"Tertiary3",
			"Tertiary4",
			"Tertiary5",
			"Tertiary6",
			"Surface1",
			"Surface2",
			"Surface3",
			"Surface4",
			"Surface5",
			"Surface6",
			"Warning",
			"Error",
			"Loss1",
			"Loss2",
			"Loss3",
			"Loss4",
			"Loss5",
			"Loss6",
			"Gain1",
			"Gain2",
			"Gain3",
			"Gain4",
			"Gain5",
			"Gain6",
			"Dark1",
			"Dark2",
			"Dark3",
			"Dark4",
			"Dark5",
			"Dark6",
			"Light1",
			"Light2",
			"Light3",
			"Light4",
			"Light5",
			"Light6",
		},
		GuiDensityModifier = {
			"Default",
			"High",
			"Low",
		},
		GuiThemeType = {
			"Dark",
			"Light",
		},
		GuiTypography = {
			"Overline",
			"Caption",
			"Body2",
			"Body1",
			"Subtitle2",
			"Subtitle1",
			"Button",
			"Headline6",
			"Headline5",
			"Headline4",
			"Headline3",
			"Headline2",
			"Headline1",
		},
		VendorPurchaseItemType = {
			"Refreshment",
			"Apartment",
		},
	},
	use_union_types_for_export = true,
	use_union_types_for_tree = false,
}
