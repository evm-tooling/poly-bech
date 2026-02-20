package tree_sitter_polybench_test

import (
	"testing"

	tree_sitter "github.com/smacker/go-tree-sitter"
	"github.com/tree-sitter/tree-sitter-polybench"
)

func TestCanLoadGrammar(t *testing.T) {
	language := tree_sitter.NewLanguage(tree_sitter_polybench.Language())
	if language == nil {
		t.Errorf("Error loading Polybench grammar")
	}
}
