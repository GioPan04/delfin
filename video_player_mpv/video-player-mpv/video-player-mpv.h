#pragma once

#include <gtk/gtk.h>

G_BEGIN_DECLS

#define VPM_VIDEO_PLAYER_MPV_TYPE (video_player_mpv_get_type())

#define VPM_VIDEO_PLAYER_MPV(obj)                                              \
  (G_TYPE_CHECK_INSTANCE_CAST((obj), VPM_VIDEO_PLAYER_MPV_TYPE, VideoPlayerMpv))
#define VPM_VIDEO_PLAYER_MPV_CLASS(klass)                                      \
  (G_TYPE_CHECK_CLASS_CAST(klass), VPM_VIDEO_PLAYER_MPV_TYPE,                  \
   VideoPlayerMpvClass)

#define VPM_IS_VIDEO_PLAYER_MPV(obj) (G_TYPE_CHECK_INSTANCE_TYPE((obj), VPM_VIDEO_PLAYER_MPV_TYPE)
#define VPM_IS_VPM_VIDEO_PLAYER_MPV_CLASS(klass)                               \
  (G_TYPE_CHECK_CLASS_TYPE((klass), VPM_VIDEO_PLAYER_MPV_TYPE))

typedef struct _VpmVideoPlayerMpv {
  GtkGLArea parent;
} VpmVideoPlayerMpv;

typedef struct _VpmVideoPlayerMpvClass {
  GtkGLAreaClass parent_class;
} VpmVideoPlayerMpvClass;

GType vpm_video_player_mpv_get_type(void);
GtkWidget *vpm_video_player_mpv_new();
gboolean vpm_video_player_mpv_class_render(GtkGLArea *widget,
                                           GdkGLContext *ctx);

G_END_DECLS
