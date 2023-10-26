#pragma once

#include "video-player-mpv/track.h"
#include <gtk/gtk.h>
#include <stdbool.h>
#include <sys/types.h>

G_BEGIN_DECLS

G_DECLARE_FINAL_TYPE(VpmVideoPlayerMpv, vpm_video_player_mpv, VPM,
                     VIDEO_PLAYER_MPV, GtkGLArea)

GtkWidget *vpm_video_player_mpv_new();

void vpm_video_player_mpv_play_uri(VpmVideoPlayerMpv *self, const char *uri);
void vpm_video_player_mpv_play(VpmVideoPlayerMpv *self);
void vpm_video_player_mpv_pause(VpmVideoPlayerMpv *self);
void vpm_video_player_mpv_stop(VpmVideoPlayerMpv *self);

double vpm_video_player_mpv_position(VpmVideoPlayerMpv *self);
void vpm_video_player_mpv_seek_to(VpmVideoPlayerMpv *self, uint seconds);
void vpm_video_player_mpv_seek_by(VpmVideoPlayerMpv *self, int seconds);

bool vpm_video_player_mpv_mute(VpmVideoPlayerMpv *self);
void vpm_video_player_mpv_set_mute(VpmVideoPlayerMpv *self, bool mute);
double vpm_video_player_mpv_volume(VpmVideoPlayerMpv *self);
void vpm_video_player_mpv_set_volume(VpmVideoPlayerMpv *self, double volume);

int vpm_video_player_mpv_current_audio_track(VpmVideoPlayerMpv *self);
void vpm_video_player_mpv_set_audio_track(VpmVideoPlayerMpv *self,
                                          uint audio_track_id);

int vpm_video_player_mpv_current_subtitle_track(VpmVideoPlayerMpv *self);
void vpm_video_player_mpv_set_subtitle_track(VpmVideoPlayerMpv *self,
                                             uint subtitle_track_id);

G_END_DECLS
