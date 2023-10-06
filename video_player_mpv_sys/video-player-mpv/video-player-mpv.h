#pragma once

#include <gtk/gtk.h>

G_BEGIN_DECLS

G_DECLARE_FINAL_TYPE(VpmVideoPlayerMpv, vpm_video_player_mpv, VPM,
                     VIDEO_PLAYER_MPV, GtkGLArea)

GtkWidget *vpm_video_player_mpv_new();
gboolean vpm_video_player_mpv_class_render(GtkGLArea *widget,
                                           GdkGLContext *ctx);

G_END_DECLS
