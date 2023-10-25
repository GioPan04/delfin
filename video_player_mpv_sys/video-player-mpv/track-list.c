#include "track-list.h"

#include <gtk/gtk.h>

G_DEFINE_BOXED_TYPE(VpmTrackList, vpm_track_list, vpm_track_list_copy,
                    vpm_track_list_free)

VpmTrackList *vpm_track_list_copy(VpmTrackList *l) {
  // TODO
  VpmTrackList *new = g_new(VpmTrackList, 1);
  new->tracks = l->tracks;
  return new;
}

void vpm_track_list_free(VpmTrackList *self) {
  // TODO
  g_free(self);
}

uint vpm_track_list_len(const VpmTrackList *self) {
  return g_list_length(self->tracks);
}

gboolean vpm_track_list_is_empty(const VpmTrackList *self) {
  return vpm_track_list_len(self) == 0;
}

VpmTrack *vpm_track_list_track(const VpmTrackList *self, uint index) {
  return vpm_track_copy(g_list_nth_data(self->tracks, index));
}
