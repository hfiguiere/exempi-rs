initSidebarItems({"constant":[["ARRAY_IS_ALT","Array is alternate values"],["ARRAY_IS_ALTTEXT","Array is alternate text."],["ARRAY_IS_ORDERED","Array is a ordered."],["ARRAY_NONE","Default value, no option."],["CLOSE_NONE","No close option"],["CLOSE_SAFE_UPDATE","Write into a temporary file and swap for crash safety."],["FORMAT_ALLOWS_ONLY_XMP","Allows access to just the XMP, ignoring other forms."],["FORMAT_ALLOW_SAFE_UPDATE","The file handler allows crash-safe file updates."],["FORMAT_CAN_EXPAND","Can expand XMP or other metadata in an existing file."],["FORMAT_CAN_INJECT_XMP","Can inject first-time XMP into an existing file."],["FORMAT_CAN_RECONCILE","Supports reconciliation between XMP and other forms."],["FORMAT_CAN_REWRITE","Can copy one file to another, writing new metadata."],["FORMAT_FOLDER_BASED_FORMAT","The format is folder oriented, for example the P2 video format."],["FORMAT_HANDLER_OWNS_FILE","The file handler does the file open and close."],["FORMAT_NEEDS_READONLY_PACKET","The file format needs the XMP packet to be read-only."],["FORMAT_NONE",""],["FORMAT_PREFERS_IN_PLACE","Can expand, but prefers in-place update."],["FORMAT_RETURNS_RAW_PACKET","File handler returns raw XMP packet information."],["FORMAT_USE_SIDECAR_XMP","The file handler uses a \"sidecar\" file for the XMP."],["IMPL_RESERVED_MASK","Reserved for transient use by the implementation."],["ITEM_IS_ARRAY","The value is an array (RDF alt/bag/seq)."],["ITEM_IS_STRUCT","The value is a structure with nested fields."],["ITEM_NONE",""],["ITER_ALIASES","Iterate the global alias table."],["ITER_CLASS_MASK","The low 8 bits are an enum of what data structure to iterate."],["ITER_INCLUDE_ALIASES","Include aliases, default is justactual properties."],["ITER_JUST_CHILDREN","Just do the immediate children of the root, default is subtree."],["ITER_JUST_LEAF_NAME","Return just the leaf part of the path, default is the full path."],["ITER_JUST_LEAF_NODES","Just do the leaf nodes, default is all nodes in the subtree."],["ITER_NAMESPACES","Iterate the global namespace table."],["ITER_NONE","No iterator flag"],["ITER_OMIT_QUALIFIERS","Omit all qualifiers."],["ITER_PROPERTIES","Iterate the property tree of a Xmp object."],["ITER_SKIP_NONE","Not flags."],["ITER_SKIP_SIBLINGS","Skip the subtree below and remaining siblings of the current node."],["ITER_SKIP_SUBTREE","Skip the subtree below the current node."],["OPEN_CACHE_TNAIL","Cache thumbnail if possible, GetThumbnail will be called."],["OPEN_FOR_UPDATE","Open for reading and writing."],["OPEN_IN_BACKGROUND","Set if calling from background thread."],["OPEN_LIMITED_SCANNING","Only packet scan files \"known\" to need scanning."],["OPEN_NONE","No open option"],["OPEN_ONLY_XMP","Only the XMP is wanted, allows space/time optimizations."],["OPEN_OPTIMIZE_FILE_LAYOUT","Optimize MPEG4 to support stream when updating This can take some time"],["OPEN_READ","Open for read-only access."],["OPEN_REPAIR_FILE","Attempt to repair a file opened for update, default is to not open (throw an exception)."],["OPEN_STRICTLY","Be strict about locating XMP and reconciling with other forms."],["OPEN_USE_PACKET_SCANNING","Force packet scanning, don't use a smart handler."],["OPEN_USE_SMART_HANDLER","Require the use of a smart handler."],["PROP_ARRAY_FORM_MASK",""],["PROP_ARRAY_INSERT_AFTER","Used by array functions. */"],["PROP_ARRAY_INSERT_BEFORE","Used by array functions."],["PROP_ARRAY_IS_ALT","Implies XMP_PropArrayIsOrdered, items are alternates."],["PROP_ARRAY_IS_ALTTEXT","Additional struct and array options. Implies kXMP_PropArrayIsAlternate, items are localized text."],["PROP_ARRAY_IS_ORDERED","Implies XMP_PropValueIsArray, item order matters."],["PROP_ARRAY_IS_UNORDERED","The item order does not matter.*/"],["PROP_COMPOSITE_MASK","Is it simple or composite (array or struct)?"],["PROP_HAS_ALIASES","This property is the base value for a set of aliases."],["PROP_HAS_LANG","Implies XMP_PropHasQualifiers, property has xml:lang."],["PROP_HAS_QUALIFIERS","Options relating to qualifiers attached to a property. The property has qualifiers, includes rdf:type and xml:lang."],["PROP_HAS_TYPE","Implies XMP_PropHasQualifiers, property has rdf:type."],["PROP_IS_ALIAS","This property is an alias name for another property."],["PROP_IS_DERIVED","This property is derived from the document content."],["PROP_IS_INTERNAL","This property is an \"internal\" property, owned by applications."],["PROP_IS_QUALIFIER","This is a qualifier, includes rdf:type and xml:lang."],["PROP_IS_STABLE","This property is not derived from the document content."],["PROP_NONE","The property has no bit set."],["PROP_VALUE_IS_ARRAY","The value is an array (RDF alt/bag/seq)."],["PROP_VALUE_IS_STRUCT","The value is a structure with nested fields."],["PROP_VALUE_IS_URI","The value is a URI, use rdf:resource attribute. DISCOURAGED"],["SERIAL_ENCODEUTF16BIG","Serialize to UTF-16 BE (big endian)"],["SERIAL_ENCODEUTF16LITTLE","Serialize to UTF-16 LE (little endian)"],["SERIAL_ENCODEUTF32BIG","Serialize to UTF-32 BE (big endian)"],["SERIAL_ENCODEUTF32LITTLE","Serialize to UTF-32 LE (little endian)"],["SERIAL_ENCODEUTF8","Serialize to UTF-8 (default)"],["SERIAL_ENCODINGMASK",""],["SERIAL_EXACTPACKETLENGTH","The padding parameter is the overall packet length. */"],["SERIAL_INCLUDETHUMBNAILPAD","Include a padding allowance for a thumbnail image. */"],["SERIAL_OMITALLFORMATTING","Omit all formatting whitespace. */"],["SERIAL_OMITPACKETWRAPPER","Omit the XML packet wrapper. */"],["SERIAL_READONLYPACKET","Default is a writeable packet. */"],["SERIAL_USECOMPACTFORMAT","Use a compact form of RDF. */"],["SERIAL_WRITEALIASCOMMENTS","Show aliases as XML comments. */"],["_LITTLEENDIAN_BIT",""],["_UTF16_BIT",""],["_UTF32_BIT",""]],"enum":[["Error","XMP errors."],["FileType","Public file formats."]],"fn":[["get_error","Get the last error code on the thread Set when a function return false or None."],["init","Initialize the library"],["namespace_prefix","Return the prefix for the namespace uri."],["prefix_namespace","Return the namespace uri for the prefix."],["register_namespace","Register namespace with uri and suggested prefix Returns the actual registered prefix."],["terminate","Terminate the library"]],"struct":[["ArrayFlags",""],["CloseFlags","Flag options to close files."],["DateTime","A wrapper around the C type DateTime"],["FormatOptionFlags","Result flage for file / format infos."],["ItemFlags",""],["IterFlags",""],["IterSkipFlags",""],["OpenFlags","Flag options for opening files."],["PropFlags",""],["SerialFlags",""],["Xmp",""],["XmpFile",""],["XmpIterator",""],["XmpString","The string wrapper from Exempi. It is meant to be used for output parameter. But gives you ownership of the string. Because of the way the C API of Exempi is implemented, we provide this type instead of using std::string::String to avoid copying of strings until needed. They are mostly returned in an Option<XmpString> enum. XmpString support several of the standard traits."]]});