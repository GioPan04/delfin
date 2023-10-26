#pragma once

#include <gtk/gtk.h>
#include <mpv/client.h>

G_BEGIN_DECLS

#define VPM_TYPE_TRACK (vpm_track_get_type())

typedef enum {
  VPM_TRACK_TYPE_VIDEO = 0,
  VPM_TRACK_TYPE_AUDIO = 1,
  VPM_TRACK_TYPE_SUBTITLE = 2,
} VpmTrackType;

typedef struct {
  VpmTrackType type;
  int id;
  char *title;
  char *language;
} VpmTrack;

GType vpm_track_get_type(void);
VpmTrack *vpm_track_new();
VpmTrack *vpm_track_copy(VpmTrack *t);
void vpm_track_free(VpmTrack *self);
VpmTrackType vpm_track_type(const VpmTrack *self);
int vpm_track_id(const VpmTrack *self);
char *vpm_track_title(const VpmTrack *self);
char *vpm_track_language(const VpmTrack *self);

G_END_DECLS

VpmTrack *track_node_to_track(const mpv_node trackNode);
