var latest_version = '4.15.7';


     /*******************************************************************************************************************************
     * raccourci
     *******************************************************************************************************************************/

    var Raccourci = {
        activate: function () {

            //Townbb.addBox();
            Raccourci.add();

            // Style
            $('<style id="dio_Raccourci_style"> ' +

            // Button
              '#dio_Raccourci_Sénat { background: url("https://gpfr.innogamescdn.com/images/game/main/buildings_sprite_40x40.png") no-repeat -360px 0; position: absolute; height: 30px; width: 30px; top:2px; left:250px; z-index:200; } ' +
              '#dio_Raccourci_Académie { background: url("https://gpfr.innogamescdn.com/images/game/main/buildings_sprite_40x40.png") 0 0; no-repeat; position: absolute; height: 30px; width: 30px; top:2px; left:280px; z-index:200; } ' +


              '</style>').appendTo("head");
        },
        deactivate: function () {
            $('#dio_Raccourci_style').remove();
        },
        add: function () {

			$('<a id="dio_Raccourci_Sénat"></a><a id="dio_Raccourci_Caserne"></a><a id="dio_Raccourci_Port"></a><a id="dio_Raccourci_Académie"></a><a id="dio_Raccourci_Agora"></a>').appendTo('.town_name_area');
            $("#dio_Raccourci_Sénat").click(function () {
				MainWindowFactory.openMainWindow();
			});
            $("#dio_Raccourci_Académie").click(function () {
				AcademyWindowFactory.openAcademyWindow();
			});

            // Tooltip
            $('#dio_Raccourci_Sénat').tooltip("Sénat");
            $('#dio_Raccourci_Académie').tooltip("Académie");
        },
    };