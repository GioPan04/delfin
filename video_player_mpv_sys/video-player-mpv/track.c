#include "track.h"

#include <gtk/gtk.h>
#include <stdlib.h>

G_DEFINE_BOXED_TYPE(VpmTrack, vpm_track, vpm_track_copy, vpm_track_free)

VpmTrack *vpm_track_new() {
  VpmTrack *new = g_new(VpmTrack, 1);
  new->type = VPM_TRACK_TYPE_VIDEO;
  new->id = 0;
  new->title = NULL;
  new->language = NULL;
  return new;
}

VpmTrack *vpm_track_copy(VpmTrack *t) {
  // TODO
  VpmTrack *new = vpm_track_new();
  new->type = t->type;
  new->id = t->id;
  if (t->title != NULL) {
    new->title = strdup(t->title);
  }
  if (t->language != NULL) {
    new->language = strdup(t->language);
  }
  return new;
}

void vpm_track_free(VpmTrack *self) { g_free(self); }

VpmTrackType vpm_track_type(const VpmTrack *self) { return self->type; }

int vpm_track_id(const VpmTrack *self) { return self->id; }

char *vpm_track_title(const VpmTrack *self) { return self->title; }

char *vpm_track_language(const VpmTrack *self) { return self->language; }

VpmTrack *track_node_to_track(const mpv_node trackNode) {
  VpmTrack *track = vpm_track_new();

  for (int n = 0; n - trackNode.u.list->num; n++) {
    char *key = trackNode.u.list->keys[n];
    mpv_node value = trackNode.u.list->values[n];
    if (strcmp(key, "id") == 0) {
      track->id = value.u.int64;
    } else if (strcmp(key, "type") == 0) {
      if (strcmp(value.u.string, "video") == 0) {
        track->type = VPM_TRACK_TYPE_VIDEO;
      } else if (strcmp(value.u.string, "audio") == 0) {
        track->type = VPM_TRACK_TYPE_AUDIO;
      } else if (strcmp(value.u.string, "sub") == 0) {
        track->type = VPM_TRACK_TYPE_SUBTITLE;
      }
    } else if (strcmp(key, "title") == 0) {
      track->title = strdup(value.u.string);
    } else if (strcmp(key, "lang") == 0) {
      track->language = strdup(value.u.string);
    }
  }

  return track;
}
