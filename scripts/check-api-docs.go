package main

import (
	"fmt"
	"go/ast"
	"go/parser"
	"go/token"
	"io/fs"
	"os"
	"path/filepath"
	"strings"
)

func main() {
	fset := token.NewFileSet()
	count, missing := 0, 0
	check := func(name string, pos token.Pos, comments ...*ast.CommentGroup) {
		count++
		for _, comment := range comments {
			if comment != nil && strings.TrimSpace(comment.Text()) != "" {
				return
			}
		}
		fmt.Fprintf(os.Stderr, "%s: missing API comment for %s\n", fset.Position(pos), name)
		missing++
	}
	err := filepath.WalkDir("src", func(path string, entry fs.DirEntry, err error) error {
		if err != nil {
			return err
		}
		if entry.IsDir() {
			if entry.Name() == "hosts" || entry.Name() == "protocol" {
				return filepath.SkipDir
			}
			return nil
		}
		if !strings.HasSuffix(path, ".go") || strings.HasSuffix(path, "_test.go") {
			return nil
		}
		file, err := parser.ParseFile(fset, path, nil, parser.ParseComments)
		if err != nil {
			return err
		}
		for _, declaration := range file.Decls {
			switch declaration := declaration.(type) {
			case *ast.FuncDecl:
				if !declaration.Name.IsExported() {
					continue
				}
				name := declaration.Name.Name
				if declaration.Recv != nil {
					receiver := declaration.Recv.List[0].Type
					if pointer, ok := receiver.(*ast.StarExpr); ok {
						receiver = pointer.X
					}
					identifier, ok := receiver.(*ast.Ident)
					if !ok || !identifier.IsExported() {
						continue
					}
					name = identifier.Name + "." + name
				}
				check(name, declaration.Pos(), declaration.Doc)
			case *ast.GenDecl:
				for _, specification := range declaration.Specs {
					switch specification := specification.(type) {
					case *ast.TypeSpec:
						if !specification.Name.IsExported() {
							continue
						}
						check(specification.Name.Name, specification.Pos(), specification.Doc, declaration.Doc)
						var fields *ast.FieldList
						switch value := specification.Type.(type) {
						case *ast.StructType:
							fields = value.Fields
						case *ast.InterfaceType:
							fields = value.Methods
						}
						if fields != nil {
							for _, field := range fields.List {
								for _, name := range field.Names {
									if name.IsExported() {
										check(specification.Name.Name+"."+name.Name, field.Pos(), field.Doc, field.Comment)
									}
								}
							}
						}
					case *ast.ValueSpec:
						for _, name := range specification.Names {
							if name.IsExported() {
								check(name.Name, name.Pos(), specification.Doc, declaration.Doc, specification.Comment)
							}
						}
					}
				}
			}
		}
		return nil
	})
	if err != nil {
		fmt.Fprintln(os.Stderr, err)
		os.Exit(1)
	}
	if missing > 0 {
		fmt.Fprintf(os.Stderr, "%d of %d public declarations and members need comments\n", missing, count)
		os.Exit(1)
	}
	fmt.Printf("Checked comments for %d handwritten public declarations and members.\n", count)
}
