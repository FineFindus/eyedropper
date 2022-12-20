#! /bin/bash

# This script extracts the typical xgettext invocation out of
# po/Makefile.in.in, in order for it to be available as a shell
# command without the need to autogen first. This is needed for
# translation tools such as the damn lies website.
#
# Call this from your GTK+ checkout directory, like this:
#
#   ./make-pot
#
# to generate po/gtk40.pot.
#
# Various things can be passed in by environment variables, which
# are heavily inspired by the variables used in po/Makefile.in.in:
#
# XGETTEXT - path of the xgettext binary
# top_srcdir - the location of the GTK+ checkout
# srcdir - the directory where POTFILES.in is located (defaults to
#          $top_srcdir/po or $top_srcdir/po-properties)
# GETTEXT_PACKAGE - the gettext domain, used for naming the resulting
#          .pot file (defaults to gtk40 or gtk40-properties)
# XGETTEXT_KEYWORDS - --keyword arguments to pass to xgettext

if test "$1" = "properties"; then
  echo "'properties' is no longer supported" >&2
  exit 1
fi

XGETTEXT="${XGETTEXT:-xgettext}"
top_srcdir="${top_srcdir:-.}"

srcdir="${srcdir:-$top_srcdir/po}"
GETTEXT_PACKAGE="${GETTEXT_PACKAGE:-gtk40}"
XGETTEXT_KEYWORDS="${XGETTEXT_KEYWORDS:- --keyword=_ --keyword=N_ --keyword=C_:1c,2 --keyword=NC_:1c,2 --keyword=g_dngettext:2,3 }"

$XGETTEXT --default-domain="$GETTEXT_PACKAGE" \
          --directory="$top_srcdir" \
          --msgid-bugs-address="https://gitlab.gnome.org/GNOME/gtk/-/issues/" \
          --add-comments \
          $XGETTEXT_KEYWORDS \
          --from-code=utf-8 \
          --flag=g_dngettext:2:pass-c-format \
          --flag=g_strdup_printf:1:c-format \
          --flag=g_string_printf:2:c-format \
          --flag=g_string_append_printf:2:c-format \
          --flag=g_error_new:3:c-format \
          --flag=g_set_error:4:c-format \
          --flag=g_markup_printf_escaped:1:c-format \
          --flag=g_log:3:c-format \
          --flag=g_print:1:c-format \
          --flag=g_printerr:1:c-format \
          --flag=g_printf:1:c-format \
          --flag=g_fprintf:2:c-format \
          --flag=g_sprintf:2:c-format \
          --flag=g_snprintf:3:c-format \
          --flag=g_scanner_error:2:c-format \
          --flag=g_scanner_warn:2:c-format \
          --flag=gtk_message_dialog_format_secondary_markup:2:c-format \
          --flag=gtk_message_dialog_format_secondary_text:2:c-format \
          --flag=gtk_message_dialog_new:5:c-format \
          --flag=gtk_message_dialog_new_with_markup:5:c-format \
          --files-from="$srcdir/POTFILES.in" \
        && test ! -f "$GETTEXT_PACKAGE.po" \
           || ( rm -f "$srcdir/$GETTEXT_PACKAGE.pot" \
                && mv "$GETTEXT_PACKAGE.po" "$srcdir/$GETTEXT_PACKAGE.pot" )

