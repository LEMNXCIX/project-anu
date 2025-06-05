import { useEditor, EditorContent, ReactNodeViewRenderer } from "@tiptap/react";
import { TextSelection } from "@tiptap/pm/state";
import StarterKit from "@tiptap/starter-kit";
import CodeBlock from "@tiptap/extension-code-block";
import {
  Bold,
  Italic,
  Strikethrough,
  List,
  ListOrdered,
  Code,
  Quote,
  Undo,
  Redo,
  Heading1,
  Heading2,
  Heading3,
  Copy,
  Link,
} from "lucide-react";
import { Button } from "@/components/ui/button";
import { Separator } from "@/components/ui/separator";
import {
  Tooltip,
  TooltipContent,
  TooltipProvider,
  TooltipTrigger,
} from "@/components/ui/tooltip";
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
  DialogTrigger,
} from "@/components/ui/dialog";
import { Input } from "@/components/ui/input";
import { NodeViewWrapper, NodeViewContent } from "@tiptap/react";
import { toast } from "sonner";
import Heading from "@tiptap/extension-heading";
import BulletList from "@tiptap/extension-bullet-list";
import OrderedList from "@tiptap/extension-ordered-list";
import Blockquote from "@tiptap/extension-blockquote";
import LinkExtension from "@tiptap/extension-link";
import { useState } from "react";

// Componente personalizado para el CodeBlock
const CodeBlockComponent = ({ node, updateAttributes }) => {
  const copyToClipboard = () => {
    const content = node.textContent;
    navigator.clipboard
      .writeText(content)
      .then(() => {
        toast.success("Exito", {
          description: "El código ha sido copiado al portapapeles.",
        });
      })
      .catch(() => {
        toast.success("Error", { description: "No se pudo copiar el código." });
      });
  };

  return (
    <NodeViewWrapper className="relative my-4">
      <div className="flex items-center justify-between bg-gray-900 text-white px-3 py-1 rounded-t-md">
        <span className="text-sm font-mono">[Código] Ejemplo</span>
        <Button
          variant="ghost"
          size="sm"
          onClick={copyToClipboard}
          aria-label="Copiar código"
          className="text-white hover:bg-gray-700"
        >
          <Copy className="h-4 w-4" />
        </Button>
      </div>
      <div className="bg-gray-800 text-white p-4 rounded-b-md overflow-x-auto">
        <NodeViewContent as="code" className="font-mono text-sm" />
      </div>
    </NodeViewWrapper>
  );
};

// Definimos un CodeBlock personalizado
const CustomCodeBlock = CodeBlock.extend({
  addNodeView() {
    return ReactNodeViewRenderer(CodeBlockComponent);
  },
});

const MarkdownEditor = () => {
  const [isLinkModalOpen, setIsLinkModalOpen] = useState(false);
  const [linkUrl, setLinkUrl] = useState("");
  const [linkAlias, setLinkAlias] = useState(""); // Nuevo estado para el alias

  const editor = useEditor({
    extensions: [
      StarterKit.configure({
        heading: false,
        bulletList: false,
        orderedList: false,
        blockquote: false,
      }),
      Heading.configure({ levels: [1, 2, 3] }),
      BulletList,
      OrderedList,
      Blockquote,
      CustomCodeBlock,
      LinkExtension.configure({
        openOnClick: true,
        autolink: true,
        linkOnPaste: true, // Asegura que las URLs pegadas se conviertan en enlaces
        HTMLAttributes: {
          class: "text-blue-500 underline hover:text-blue-700",
          target: "_blank",
          rel: "noopener noreferrer",
        },
        validate: (href) => {
          try {
            new URL(href);
            console.log("URL válida detectada por LinkExtension:", href);
            return true;
          } catch {
            console.warn("URL inválida:", href);
            return false;
          }
        },
      }),
    ],
    content: ` `,
    editorProps: {
      attributes: {
        class: "p-4 focus:outline-none bg-stone-900 rounded-md text-white", // Añadimos text-white para mejor visibilidad
      },
    },
    onCreate({ editor }) {
      console.log("Editor creado. Contenido inicial:", editor.getHTML());
    },
    onUpdate({ editor }) {
      console.log("Contenido actualizado:", editor.getHTML());
    },
  });

  if (!editor) {
    return null;
  }

  return (
    <TooltipProvider>
      <div className="rounded-lg shadow-sm p-4 bg-stone-900">
        {/* Toolbar */}
        <div className="flex flex-wrap items-center gap-2 border-b pb-2 mb-4">
          {/* Bold */}
          <Tooltip>
            <TooltipTrigger asChild>
              <Button
                variant={editor.isActive("bold") ? "secondary" : "ghost"}
                size="sm"
                onClick={() => editor.chain().focus().toggleBold().run()}
                aria-label="Toggle bold"
              >
                <Bold className="h-4 w-4 text-secondary-foreground" />
              </Button>
            </TooltipTrigger>
            <TooltipContent>Negrita</TooltipContent>
          </Tooltip>

          {/* Italic */}
          <Tooltip>
            <TooltipTrigger asChild>
              <Button
                variant={editor.isActive("italic") ? "secondary" : "ghost"}
                size="sm"
                onClick={() => editor.chain().focus().toggleItalic().run()}
                aria-label="Toggle italic"
              >
                <Italic className="h-4 w-4 text-secondary-foreground" />
              </Button>
            </TooltipTrigger>
            <TooltipContent>Cursiva</TooltipContent>
          </Tooltip>

          {/* Strikethrough */}
          <Tooltip>
            <TooltipTrigger asChild>
              <Button
                variant={editor.isActive("strike") ? "secondary" : "ghost"}
                size="sm"
                onClick={() => editor.chain().focus().toggleStrike().run()}
                aria-label="Toggle strikethrough"
              >
                <Strikethrough className="h-4 w-4 text-secondary-foreground" />
              </Button>
            </TooltipTrigger>
            <TooltipContent>Tachado</TooltipContent>
          </Tooltip>

          <Separator orientation="vertical" className="w-[1px] h-6 mx-1" />

          {/* Headings */}
          <Tooltip>
            <TooltipTrigger asChild>
              <Button
                variant={
                  editor.isActive("heading", { level: 1 })
                    ? "secondary"
                    : "ghost"
                }
                size="sm"
                onClick={() =>
                  editor.chain().focus().toggleHeading({ level: 1 }).run()
                }
                aria-label="Toggle heading 1"
              >
                <Heading1 className="h-4 w-4 text-secondary-foreground" />
              </Button>
            </TooltipTrigger>
            <TooltipContent>Título 1</TooltipContent>
          </Tooltip>

          <Tooltip>
            <TooltipTrigger asChild>
              <Button
                variant={
                  editor.isActive("heading", { level: 2 })
                    ? "secondary"
                    : "ghost"
                }
                size="sm"
                onClick={() =>
                  editor.chain().focus().toggleHeading({ level: 2 }).run()
                }
                aria-label="Toggle heading 2"
              >
                <Heading2 className="h-4 w-4 text-secondary-foreground" />
              </Button>
            </TooltipTrigger>
            <TooltipContent>Título 2</TooltipContent>
          </Tooltip>

          <Tooltip>
            <TooltipTrigger asChild>
              <Button
                variant={
                  editor.isActive("heading", { level: 3 })
                    ? "secondary"
                    : "ghost"
                }
                size="sm"
                onClick={() =>
                  editor.chain().focus().toggleHeading({ level: 3 }).run()
                }
                aria-label="Toggle heading 3"
              >
                <Heading3 className="h-4 w-4 text-secondary-foreground" />
              </Button>
            </TooltipTrigger>
            <TooltipContent>Título 3</TooltipContent>
          </Tooltip>

          <Separator orientation="vertical" className="w-[1px] h-6 mx-1" />

          {/* Lists */}
          <Tooltip>
            <TooltipTrigger asChild>
              <Button
                variant={editor.isActive("bulletList") ? "secondary" : "ghost"}
                size="sm"
                onClick={() => editor.chain().focus().toggleBulletList().run()}
                aria-label="Toggle bullet list"
              >
                <List className="h-4 w-4 text-secondary-foreground" />
              </Button>
            </TooltipTrigger>
            <TooltipContent>Lista con viñetas</TooltipContent>
          </Tooltip>

          <Tooltip>
            <TooltipTrigger asChild>
              <Button
                variant={editor.isActive("orderedList") ? "secondary" : "ghost"}
                size="sm"
                onClick={() => editor.chain().focus().toggleOrderedList().run()}
                aria-label="Toggle ordered list"
              >
                <ListOrdered className="h-4 w-4 text-secondary-foreground" />
              </Button>
            </TooltipTrigger>
            <TooltipContent>Lista numerada</TooltipContent>
          </Tooltip>

          <Separator orientation="vertical" className="w-[1px] h-6 mx-1" />

          {/* Code Block */}
          <Tooltip>
            <TooltipTrigger asChild>
              <Button
                variant={editor.isActive("codeBlock") ? "secondary" : "ghost"}
                size="sm"
                onClick={() => editor.chain().focus().toggleCodeBlock().run()}
                aria-label="Toggle code block"
              >
                <Code className="h-4 w-4 text-secondary-foreground" />
              </Button>
            </TooltipTrigger>
            <TooltipContent>Bloque de código</TooltipContent>
          </Tooltip>

          {/* Blockquote */}
          <Tooltip>
            <TooltipTrigger asChild>
              <Button
                variant={editor.isActive("blockquote") ? "secondary" : "ghost"}
                size="sm"
                onClick={() => editor.chain().focus().toggleBlockquote().run()}
                aria-label="Toggle blockquote"
              >
                <Quote className="h-4 w-4 text-secondary-foreground" />
              </Button>
            </TooltipTrigger>
            <TooltipContent>Cita</TooltipContent>
          </Tooltip>

          {/* Link */}
          <Tooltip>
            <TooltipTrigger asChild>
              <Dialog open={isLinkModalOpen} onOpenChange={setIsLinkModalOpen}>
                <DialogTrigger asChild>
                  <Button
                    variant={editor.isActive("link") ? "secondary" : "ghost"}
                    size="sm"
                    onClick={() => {
                      if (editor.isActive("link")) {
                        console.log("Eliminando enlace existente...");
                        editor.chain().focus().unsetLink().run();
                        setIsLinkModalOpen(false);
                        console.log(
                          "Contenido después de eliminar enlace:",
                          editor.getHTML()
                        );
                      }
                    }}
                    aria-label={
                      editor.isActive("link") ? "Remove link" : "Insert link"
                    }
                  >
                    <Link className="h-4 w-4 text-secondary-foreground" />
                  </Button>
                </DialogTrigger>
                <DialogContent className="sm:max-w-[425px]">
                  <DialogHeader>
                    <DialogTitle>Insertar enlace</DialogTitle>
                    <DialogDescription>
                      Ingresa la URL y, opcionalmente, un alias para el enlace.
                    </DialogDescription>
                  </DialogHeader>
                  <div className="space-y-4">
                    <Input
                      value={linkUrl}
                      onChange={(e) => setLinkUrl(e.target.value)}
                      placeholder="https://ejemplo.com"
                      autoFocus
                    />
                    <Input
                      value={linkAlias}
                      onChange={(e) => setLinkAlias(e.target.value)}
                      placeholder="Texto visible (opcional)"
                    />
                  </div>
                  <DialogFooter>
                    <Button
                      variant="outline"
                      onClick={() => {
                        setIsLinkModalOpen(false);
                        setLinkUrl("");
                        setLinkAlias(""); // Limpiamos el alias al cancelar
                      }}
                    >
                      Cancelar
                    </Button>
                    <Button
                      onClick={() => {
                        console.log(
                          "Insertando enlace con URL:",
                          linkUrl,
                          "y alias:",
                          linkAlias
                        );
                        if (linkUrl) {
                          const normalizedUrl = linkUrl.trim();
                          const aliasText = linkAlias.trim() || normalizedUrl; // Usamos el alias si existe, si no, la URL
                          console.log(
                            "Contenido antes de insertar:",
                            editor.getHTML()
                          );
                          const selectedText =
                            editor.state.selection.content().content.firstChild
                              ?.textContent || "";

                          if (!selectedText) {
                            // Si no hay texto seleccionado, inserta el alias o la URL
                            editor
                              .chain()
                              .focus()
                              .insertContent(aliasText)
                              .command(({ tr }) => {
                                const { from } = tr.selection;
                                tr.setSelection(
                                  TextSelection.create(
                                    tr.doc,
                                    from - aliasText.length,
                                    from
                                  )
                                );
                                return true;
                              })
                              .setLink({ href: normalizedUrl })
                              .run();
                          } else {
                            // Si hay texto seleccionado, aplica el enlace directamente
                            editor
                              .chain()
                              .focus()
                              .setLink({ href: normalizedUrl })
                              .run();
                          }

                          console.log(
                            "Estado del enlace después de insertar:",
                            editor.isActive("link")
                          );
                          console.log(
                            "Contenido después de insertar:",
                            editor.getHTML()
                          );
                        } else {
                          console.log("No se ingresó una URL.");
                        }
                        setIsLinkModalOpen(false);
                        setLinkUrl("");
                        setLinkAlias(""); // Limpiamos el alias después de insertar
                      }}
                    >
                      Insertar
                    </Button>
                  </DialogFooter>
                </DialogContent>
              </Dialog>
            </TooltipTrigger>
            <TooltipContent>Insertar enlace</TooltipContent>
          </Tooltip>

          <Separator orientation="vertical" className="w-[1px] h-6 mx-1" />

          {/* Undo/Redo */}
          <Tooltip>
            <TooltipTrigger asChild>
              <Button
                variant="ghost"
                size="sm"
                onClick={() => editor.chain().focus().undo().run()}
                disabled={!editor.can().undo()}
                aria-label="Undo"
              >
                <Undo className="h-4 w-4 text-secondary-foreground" />
              </Button>
            </TooltipTrigger>
            <TooltipContent>Deshacer</TooltipContent>
          </Tooltip>

          <Tooltip>
            <TooltipTrigger asChild>
              <Button
                variant="ghost"
                size="sm"
                onClick={() => editor.chain().focus().redo().run()}
                disabled={!editor.can().redo()}
                aria-label="Redo"
              >
                <Redo className="h-4 w-4 text-secondary-foreground" />
              </Button>
            </TooltipTrigger>
            <TooltipContent>Rehacer</TooltipContent>
          </Tooltip>
        </div>

        {/* Editor Content */}
        <EditorContent
          editor={editor}
          className="prose prose-invert max-w-none w-full focus:outline-none h-60 text-white" // Corregimos h=60 a h-60 y añadimos prose-invert para temas oscuros
        />
        {/* Estilos personalizados para asegurar que los elementos se visualicen */}
      </div>
    </TooltipProvider>
  );
};

export default MarkdownEditor;
