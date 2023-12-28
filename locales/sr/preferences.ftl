prefs-vp-on-left-click-options =
    .play-pause = Пусти/заустави видео
    .toggle-controls = Прикажи/сакриј контроле
prefs-vp-interface = Кориснички приказ
prefs-general-page = Опште
prefs-skip-amount =
    { $seconds } { $seconds ->
        [one] секунда
       *[other] секунди
    }
prefs-window-title = Подешавања
prefs-vp-subs = Титлови
prefs-vp-intro-skipper-auto-skip =
    .title = Аутомаски прескочи уводне шпице
    .subtitle = Прескочи уводне шпице без притискања дугмета
prefs-general-language =
    .title = Језик
    .subtitle = Помозите са превођењем на <a href="{ $weblateUrl }">Weblate</a>
    .option-default = Подразумевано по систему ({ $languageId })
prefs-vp-subs-position =
    .title = Положај титла
    .subtitle = 0 је врх екрана, а 100 је дно
prefs-vp-subs-scale =
    .title = Вечина титлова
    .subtitle = Фактор повећања величине титлова
prefs-vp-skip-forwards =
    .title = Колико премотати унапред
    .subtitle = Колико секунди премотати унапред одједном
prefs-vp-on-left-click =
    .title = Када се притисне тастер миша
    .subtitle = Шта да се деси када се притисне леви тастер миша унутар видео плејера
prefs-vp-subs-font =
    .title = Фонт титлова
    .subtitle = Приказани су само подржани фонтови
prefs-vp-other = Остало
prefs-vp-skip-backwards =
    .title = Колико премотати уназад
    .subtitle = Колико секунди премотати уназад одједном
prefs-vp-subs-style = Стил титлова
prefs-vp-subs-colour =
    .title = Боја текста титлова
prefs-vp-subs-background-colour =
    .title = Боја позадине титлова
prefs-vp-page = Видео плејер
prefs-vp-plugins = Додаци
prefs-general-theme =
    .title = Тема
    .option-default = Системски подразумевана
    .option-light = Светла
    .option-dark = Тамна
prefs-vp-subs-more = Додатна подешавања за титлове
prefs-vp-subs-style-ass-warning =
    Ова подешавања не важе за титлове типа ASS/SSA.
    За ASS/SSA, идите на "{ prefs-vp-subs-more }".
prefs-vp-subs-reset =
    .label = Ресетуј
    .tooltip = Ресетуј подешавања за титлове
prefs-vp-subs-more-ass-warning =
    .title = Ово су подешавања за титлове типа ASS/SSA!
    .subtitle =
        Титлови ASS/SSA често садрже посебне стилове и позиционирање. Промене ових подешавања могу довести до тога да се не приказују како треба.

        Ако желите да промените излгед осталих типова титлова, препоручено је да мењате подешавања у "{ prefs-vp-subs-style }".
prefs-vp-intro-skipper =
    .title = Додатак за прескакање уводних шпица (Intro Skipper)
    .subtitle =
        Додаје дугме за премотавање уводних шпица.
        Неопходно је да додатак <a href="{ $introSkipperUrl }">Intro Skipper</a> буде инсталиран на вашем серверу.
prefs-vp-experimental =
    .title = Експериментална подешавања
    .subtitle =
        Ова подешавања могу довести до проблема.
        Није препоручљиво да их мењате.
prefs-vp-jellyscrub =
    .title = Додатак Jellyscrub
    .subtitle =
        Приказује сличице док је миш изнад траке за премотавање.
        Неопходно је да буде инсталиран додатак <a href="{ $jellyscrubUrl }">Jellyscrub</a> на серверу.
prefs-vp-hls-playback = Пуштање HLS
prefs-vp-backend =
    .title = Библиотека за пуштање снимака
    .subtitle = Да би промене биле примењене, морате рестартовати { app-name }.
    .value-mpv = MPV
    .value-gstreamer = GStreamer
