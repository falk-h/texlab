package texlab.completion.latex.data.symbols

import org.eclipse.lsp4j.CompletionItem
import org.eclipse.lsp4j.CompletionParams
import texlab.LatexDocument
import texlab.completion.CompletionItemFactory
import texlab.completion.latex.LatexCommandProvider
import texlab.provider.FeatureRequest
import texlab.syntax.latex.LatexCommandSyntax

class LatexCommandSymbolProvider(private val database: LatexSymbolDatabase) : LatexCommandProvider() {
    override fun complete(request: FeatureRequest<CompletionParams>,
                          command: LatexCommandSyntax): List<CompletionItem> {
        request.document as LatexDocument
        return database.index.commands
                .filter { it.component == null || request.document.tree.components.contains(it.component) }
                .map { createItem(it) }
    }

    private fun createItem(symbol: LatexCommandSymbol): CompletionItem {
        return CompletionItemFactory.createCommandSymbol(
                symbol.command.substring(1),
                symbol.component,
                database.resolve(symbol.imageId))
    }
}