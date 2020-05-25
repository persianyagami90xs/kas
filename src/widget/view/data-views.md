Data Views
==========

This document contains rough development plans for "view widgets".

Premise
-----------

Many GUI frameworks use "data models" which separate a data set from a view of
that data set, where the view usually has a reference to the data set.
Should we try to do something similar? Can it already be built on top of
existing functionality?

Certain data views such as tables and spread-sheets may operate as a view over
large amounts of data. Constructing an individual widget for each row or cell is
not reasonable beyond a few thousand items (and inefficient even before this).
Theoretically a table or spread-sheet may be a single widget which directly
handles all input and drawing. It is likely more practical to use real child
widgets, but only as many as required to view whatever content will fit on the
screen.

When viewing large databases, it is not even reasonable to read all available
data, let alone try to estimate the size requirements for displaying each row;
this implies that the rect for each child must either be fixed or calculated via
a simplified model (usually, assuming a fixed size for each cell / row).

This can work with internal or external storage. In practice a copy of visible
data will usually be needed anyway, so there is little incentive for external
storage. This implies that a "table view over external data" is equivalent to a
"table built dynamically from external data" with some logic to pre-allocate a
set number of rows, optionally hide rows and move data between rows.
Allowing responsive scrolling may require several hidden rows as a cache and
the table-widget being able to request additional data rows during event
handling. Keyboard navigation may require some hacks if unused child widgets
are moved directly between the top and bottom of the view range when scrolling.


Scope
-----

### Data models

The user should be able to implement a data-model interface around an in-memory
data-set.

There should be support for "server-side" queries: search/filter, sort, paging.

This model may extend to use of external databases, but will likely require
attention to how caching and queries work and should use a custom row
identifier type (e.g. for events reporting a row selection).

### Views

What kind of view widgets do we want?

-   a simple list
-   a table with fixed or variable columns and optional headers
-   a tree view
-   a 2D grid (spreadsheet)
-   a "flow grid"

In general, the atoms are a row or a cell. The user should provide this type,
with the ability to set itself from a (serialised) data atom, serialise itself
to a data atom, and do the usual widget stuff (with user-defined message type).

#### Content

What content should be possible within a view?

-   plain text
-   formatted text
-   icons / general graphics
-   buttons, checkboxes / general widgets?

#### Operations

What kind of operations should be possible on this data? This needs to be
configurable.

-   selecting an item
-   selecting multiple items
-   rearranging items
-   sorting by a column
-   editing a text-entry in place




Model-view separation
----------------------

Notation: lets name a widget displaying data from an external source a **view**.

### Properties unique to the view

Despite using an external data source, view widgets must have several data
fields of their own, as follows.

Views should all support scrolling, thus have a scroll offset (pixels or rows),
as well as the option to hide scrollbars.

A view over a large dataset will likely only bother creating widgets for some
window around the subset visible on the screen, since creating millions of
child widgets has performance issues, thus the view must store meta-info about
this window and quite possibly cache all data from the window.

Views may optionally support selecting one or multiple items. *Probably* this
selection info is a property of the view, not the leaf widgets, though the
leaves may be responsible for rendering this (passed like the `disabled` state).

A tree view needs to know whether to expand child items in the tree and
will probably cache the location of sub-lists.

A view might *possibly* also store the sort-order of items or coordinates of
icons, *but* this may be a bad idea.

Editing data in a practical manner requires a local copy, e.g. within an
`EditBox` widget.

### Data retrieval and model communication

The view must be able to access data:

-   to determine how many rows are available
-   to set a complex sub-widget like `EditBox` both initially and when
    the view window changes
-   when drawing
-   potentially in `size_rules`

Additionally:

-   a model must be able to notify a view of updates from external sources
-   a view must be able to update data (e.g. via a check-box or edit-box)

Assuming that a parent widget provides access to the data (directly or via a
user-supplied synchronisation mechanism), the logical way to do this is:

-   methods on the view that the parent may call in order to:
    
    -   initialise or fully-refresh the view
    -   notify the view of a specific update
    -   programmatically read or set selection and expansion info

-   messages returned via a specific `Handler::Msg` type

#### Access for `draw` and `size_rules`

The simplest option is to use standard widgets like `Label` supplied with a
copy of the data. Any other option would be insanely complex for an `EditBox`,
thus we might as well start with this option for everything.
Below we describe an alternative for static (not editable) data.

Theoretically a reference to data may be provided to `draw`(and maybe
`size_rules`), avoiding the need for a local copy.
This could be type-safe by giving widgets an associated data type.

This may be extended to allow "instancing" of data views by passing a data slice
and index.

This can all be generalised by templating widgets over a "data accessor", with
an embedded data field and associated types for the data reference and index,
allowing the same "Label" widget to be used with internal or external storage
— however this may be far more complexity than is warranted.

An alternative, for larger data items, would be use of `Cow` or a key into an
external data set (e.g. of sprites).

### Remote data sets and caching

The implementation of the data model may retrieve data from a remote database.
For practical reasons, more data should be cached when using a remote provider
than with a local (in-memory) provider. This implies (a) that if data is passed
into the `draw` method instead of cached internally, this must be cached
externally, and (b) that predictive caching should be possible for smoother
scrolling, implying that either the view must support this directly or it should
provide enough hints to enable it.

The simplest path here is (1) use internal caching and (2) have an option to
enable predictive caching within the view (perhaps with an absolute number
limiting the number of rows to cache outside the visible window).

### Sorting, filtering and item identifiers

The simplest approach here would be to access all data via an index in a range
and handle sorting and filtering externally. This may make filtering expensive.
Additionally, we need to be careful that any edits made via the view get applied
to the correct row.


Implementation plan
-------------------------

1.  Add a `ListView` widget based on `ScrollRegion<Column<Label>>`.
    Build the whole view on configure and refresh.
2.  Use only enough child widgets for the visible window and re-allocate them
    when scrolling.
3.  Support selection of items, where selection is a property of the view.
    (May require changes to `Layout::draw`.)
4.  Add a `FixedRowLayout` or some such and support multiple columns of text.
5.  Add headers; allow requesting sorting of the data set.
6.  Support user-defined rows over a user-defined (row-based) data model.
7.  Add example with a delay to data requests simulating remote data access.
    Tune the view for responsiveness with async data retrieval.
8.  Plan next steps: tree views, flow views, 2D cellular (spreadsheet) views
