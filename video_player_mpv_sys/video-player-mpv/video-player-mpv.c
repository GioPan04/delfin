#include "video-player-mpv.h"

#include <epoxy/gl.h>
#include <gtk/gtk.h>
#include <stdio.h>

struct _VpmVideoPlayerMpv {
  GtkGLArea parent;
};

G_DEFINE_TYPE(VpmVideoPlayerMpv, vpm_video_player_mpv, GTK_TYPE_GL_AREA);

static void vpm_video_player_mpv_class_init(VpmVideoPlayerMpvClass *class) {
  GtkGLAreaClass *gl_area_class = (GtkGLAreaClass *)class;
  gl_area_class->render = vpm_video_player_mpv_class_render;
}

static void vpm_video_player_mpv_init(VpmVideoPlayerMpv *widget) {}

GtkWidget *vpm_video_player_mpv_new() {
  return g_object_new(vpm_video_player_mpv_get_type(), NULL);
}

gboolean vpm_video_player_mpv_class_render(GtkGLArea *widget,
                                           GdkGLContext *ctx) {
  glClearColor(0, 1, 0, 1);
  glClear(GL_COLOR_BUFFER_BIT);
  return TRUE;
}
