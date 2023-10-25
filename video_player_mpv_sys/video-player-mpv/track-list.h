#pragma once

#include <gtk/gtk.h>

#include "track.h"

G_BEGIN_DECLS

#define VPM_TYPE_TRACK_LIST (vpm_track_list_get_type())

typedef struct {
  GList *tracks;
} VpmTrackList;

GType vpm_track_list_get_type(void);
VpmTrackList *vpm_track_list_copy(VpmTrackList *t);
void vpm_track_list_free(VpmTrackList *self);
uint vpm_track_list_len(const VpmTrackList *self);
gboolean vpm_track_list_is_empty(const VpmTrackList *self);
VpmTrack *vpm_track_list_track(const VpmTrackList *self, uint index);

G_END_DECLS
