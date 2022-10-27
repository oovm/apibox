// This file was generated by Peginator v0.3.0
// Hash of the grammar file: 6B6B1DD20181DEED4C23A814A63A777B91751769B6BCB613BD7C009B535EECB7
// Any changes to it will be lost on regeneration

#[derive(Debug, Clone)]
pub struct VosParser {
    pub statements: Vec<VosStatementNode>,
}
#[allow(non_camel_case_types)]
#[derive(Debug, Clone)]
pub enum VosStatementNode {
    DeclareStatementNode(DeclareStatementNode),
    DefineStatementNode(DefineStatementNode),
}
#[derive(Debug, Clone)]
pub struct DeclareStatementNode {
    pub modifiers: Vec<ModifierNode>,
    pub id: IdentifierNode,
    pub body: DeclareBodyNode,
    pub position: std::ops::Range<usize>,
}
#[derive(Debug, Clone)]
pub struct DefineStatementNode {
    pub modifiers: Vec<ModifierNode>,
    pub id: IdentifierNode,
    pub body: DeclareBodyNode,
    pub position: std::ops::Range<usize>,
}
#[derive(Debug, Clone)]
pub struct DeclareBodyNode {
    pub id: Vec<IdentifierNode>,
    pub position: std::ops::Range<usize>,
}
#[derive(Debug, Clone)]
pub struct NumNode {
    pub string: String,
    pub position: std::ops::Range<usize>,
}
#[derive(Debug, Clone)]
pub struct ModifierNode {
    pub string: String,
    pub position: std::ops::Range<usize>,
}
#[derive(Debug, Clone)]
pub struct IdentifierNode {
    pub string: String,
    pub position: std::ops::Range<usize>,
}
pub type XID_START = char;
pub type XID_CONTINUE = char;
impl peginator_generated::PegParser for VosParser {
    fn parse_advanced<T: peginator_generated::ParseTracer>(
        s: &str,
        settings: &peginator_generated::ParseSettings,
    ) -> Result<Self, peginator_generated::ParseError> {
        Ok(peginator_generated::parse_VosParser(
            peginator_generated::ParseState::new(s, settings),
            T::new(),
            &mut Default::default(),
        )?
        .result)
    }
}
#[allow(non_snake_case, unused_variables, unused_imports, unused_mut, dead_code)]
mod peginator_generated {
    use super::*;
    use peginator::runtime::*;
    pub use peginator::runtime::{IndentedTracer, ParseError, ParseSettings, ParseState, ParseTracer, PegParser, PegPosition};
    #[derive(Default)]
    pub struct ParseCache<'a> {
        _please_dont_complain: std::marker::PhantomData<&'a ()>,
    }
    mod VosParser_impl {
        use super::*;
        mod part_0 {
            use super::*;
            mod closure {
                use super::*;
            }
            #[inline(always)]
            pub fn parse<'a>(
                state: ParseState<'a>,
                tracer: impl ParseTracer,
                cache: &mut ParseCache<'a>,
            ) -> ParseResult<'a, Parsed> {
                let mut iterations: usize = 0;
                let mut state = state;
                let mut statements: Vec<VosStatementNode> = Vec::new();
                loop {
                    match parse_Whitespace(state.clone(), tracer, cache)
                        .and_then(|ParseOk { state, .. }| parse_VosStatementNode(state, tracer, cache))
                        .map_inner(|result| vec![result])
                    {
                        Ok(ParseOk { result: __result, state: new_state, .. }) => {
                            statements.extend(__result);
                            state = new_state;
                        }
                        Err(err) => {
                            state = state.record_error(err);
                            break;
                        }
                    }
                    iterations += 1;
                }
                Ok(ParseOk { result: statements, state })
            }
            pub type Parsed = Vec<VosStatementNode>;
        }
        #[inline(always)]
        pub fn parse<'a>(
            state: ParseState<'a>,
            tracer: impl ParseTracer,
            cache: &mut ParseCache<'a>,
        ) -> ParseResult<'a, Parsed> {
            let ParseOk { result: mut statements, state } = part_0::parse(state, tracer, cache)?;
            let ParseOk { state, .. } =
                parse_Whitespace(state, tracer, cache).and_then(|ParseOk { state, .. }| parse_end_of_input(state))?;
            Ok(ParseOk { result: statements, state })
        }
        pub type Parsed = Vec<VosStatementNode>;
        #[inline(always)]
        pub fn rule_parser<'a>(
            state: ParseState<'a>,
            tracer: impl ParseTracer,
            cache: &mut ParseCache<'a>,
        ) -> ParseResult<'a, super::VosParser> {
            let result = parse(state, tracer, cache)?.map(|r| super::VosParser { statements: r });
            Ok(result)
        }
    }
    #[inline]
    pub(super) fn parse_VosParser<'a>(
        state: ParseState<'a>,
        tracer: impl ParseTracer,
        cache: &mut ParseCache<'a>,
    ) -> ParseResult<'a, VosParser> {
        tracer.run_traced("VosParser", state, |state, tracer| VosParser_impl::rule_parser(state, tracer, cache))
    }
    mod VosStatementNode_impl {
        use super::*;
        #[inline(always)]
        pub fn parse<'a>(
            mut state: ParseState<'a>,
            tracer: impl ParseTracer,
            cache: &mut ParseCache<'a>,
        ) -> ParseResult<'a, Parsed> {
            ChoiceHelper::new(state)
                .choice(|state| {
                    parse_Whitespace(state, tracer, cache)
                        .and_then(|ParseOk { state, .. }| parse_DeclareStatementNode(state, tracer, cache))
                        .map_inner(Parsed__override::DeclareStatementNode)
                })
                .choice(|state| {
                    parse_Whitespace(state, tracer, cache)
                        .and_then(|ParseOk { state, .. }| parse_DefineStatementNode(state, tracer, cache))
                        .map_inner(Parsed__override::DefineStatementNode)
                })
                .end()
        }
        pub type Parsed = Parsed__override;
        use super::VosStatementNode as Parsed__override;
        #[inline(always)]
        pub fn rule_parser<'a>(
            state: ParseState<'a>,
            tracer: impl ParseTracer,
            cache: &mut ParseCache<'a>,
        ) -> ParseResult<'a, super::VosStatementNode> {
            let result = parse(state, tracer, cache)?;
            Ok(result)
        }
    }
    #[inline]
    pub(super) fn parse_VosStatementNode<'a>(
        state: ParseState<'a>,
        tracer: impl ParseTracer,
        cache: &mut ParseCache<'a>,
    ) -> ParseResult<'a, VosStatementNode> {
        tracer.run_traced("VosStatementNode", state, |state, tracer| VosStatementNode_impl::rule_parser(state, tracer, cache))
    }
    impl PegPosition for VosStatementNode {
        fn position(&self) -> &std::ops::Range<usize> {
            match self {
                Self::DeclareStatementNode(x) => x.position(),
                Self::DefineStatementNode(x) => x.position(),
            }
        }
    }
    mod DeclareStatementNode_impl {
        use super::*;
        mod part_0 {
            use super::*;
            mod closure {
                use super::*;
            }
            #[inline(always)]
            pub fn parse<'a>(
                state: ParseState<'a>,
                tracer: impl ParseTracer,
                cache: &mut ParseCache<'a>,
            ) -> ParseResult<'a, Parsed> {
                let mut iterations: usize = 0;
                let mut state = state;
                let mut modifiers: Vec<ModifierNode> = Vec::new();
                loop {
                    match parse_Whitespace(state.clone(), tracer, cache)
                        .and_then(|ParseOk { state, .. }| parse_ModifierNode(state, tracer, cache))
                        .map_inner(|result| vec![result])
                    {
                        Ok(ParseOk { result: __result, state: new_state, .. }) => {
                            modifiers.extend(__result);
                            state = new_state;
                        }
                        Err(err) => {
                            state = state.record_error(err);
                            break;
                        }
                    }
                    iterations += 1;
                }
                Ok(ParseOk { result: modifiers, state })
            }
            pub type Parsed = Vec<ModifierNode>;
        }
        #[inline(always)]
        pub fn parse<'a>(
            state: ParseState<'a>,
            tracer: impl ParseTracer,
            cache: &mut ParseCache<'a>,
        ) -> ParseResult<'a, Parsed> {
            let ParseOk { result: mut modifiers, state } = part_0::parse(state, tracer, cache)?;
            let ParseOk { result: id, state } = parse_Whitespace(state, tracer, cache)
                .and_then(|ParseOk { state, .. }| parse_IdentifierNode(state, tracer, cache))?;
            let ParseOk { state, .. } = parse_Whitespace(state, tracer, cache)
                .and_then(|ParseOk { state, .. }| parse_character_literal(state, '{'))
                .discard_result()?;
            let ParseOk { result: body, state } = parse_Whitespace(state, tracer, cache)
                .and_then(|ParseOk { state, .. }| parse_DeclareBodyNode(state, tracer, cache))?;
            let ParseOk { state, .. } = parse_Whitespace(state, tracer, cache)
                .and_then(|ParseOk { state, .. }| parse_character_literal(state, '}'))
                .discard_result()?;
            Ok(ParseOk { result: Parsed { modifiers, id, body }, state })
        }
        pub struct Parsed {
            pub modifiers: Vec<ModifierNode>,
            pub id: IdentifierNode,
            pub body: DeclareBodyNode,
        }
        #[inline(always)]
        pub fn rule_parser<'a>(
            state: ParseState<'a>,
            tracer: impl ParseTracer,
            cache: &mut ParseCache<'a>,
        ) -> ParseResult<'a, super::DeclareStatementNode> {
            let result = parse(state.clone(), tracer, cache)?.map_with_state(|r, new_state| super::DeclareStatementNode {
                modifiers: r.modifiers,
                id: r.id,
                body: r.body,
                position: state.range_until(new_state),
            });
            Ok(result)
        }
    }
    #[inline]
    pub(super) fn parse_DeclareStatementNode<'a>(
        state: ParseState<'a>,
        tracer: impl ParseTracer,
        cache: &mut ParseCache<'a>,
    ) -> ParseResult<'a, DeclareStatementNode> {
        tracer.run_traced("DeclareStatementNode", state, |state, tracer| {
            DeclareStatementNode_impl::rule_parser(state, tracer, cache)
        })
    }
    impl PegPosition for DeclareStatementNode {
        fn position(&self) -> &std::ops::Range<usize> {
            &self.position
        }
    }
    mod DefineStatementNode_impl {
        use super::*;
        mod part_0 {
            use super::*;
            mod closure {
                use super::*;
            }
            #[inline(always)]
            pub fn parse<'a>(
                state: ParseState<'a>,
                tracer: impl ParseTracer,
                cache: &mut ParseCache<'a>,
            ) -> ParseResult<'a, Parsed> {
                let mut iterations: usize = 0;
                let mut state = state;
                let mut modifiers: Vec<ModifierNode> = Vec::new();
                loop {
                    match parse_Whitespace(state.clone(), tracer, cache)
                        .and_then(|ParseOk { state, .. }| parse_ModifierNode(state, tracer, cache))
                        .map_inner(|result| vec![result])
                    {
                        Ok(ParseOk { result: __result, state: new_state, .. }) => {
                            modifiers.extend(__result);
                            state = new_state;
                        }
                        Err(err) => {
                            state = state.record_error(err);
                            break;
                        }
                    }
                    iterations += 1;
                }
                Ok(ParseOk { result: modifiers, state })
            }
            pub type Parsed = Vec<ModifierNode>;
        }
        #[inline(always)]
        pub fn parse<'a>(
            state: ParseState<'a>,
            tracer: impl ParseTracer,
            cache: &mut ParseCache<'a>,
        ) -> ParseResult<'a, Parsed> {
            let ParseOk { result: mut modifiers, state } = part_0::parse(state, tracer, cache)?;
            let ParseOk { result: id, state } = parse_Whitespace(state, tracer, cache)
                .and_then(|ParseOk { state, .. }| parse_IdentifierNode(state, tracer, cache))?;
            let ParseOk { state, .. } = parse_Whitespace(state, tracer, cache)
                .and_then(|ParseOk { state, .. }| parse_character_literal(state, '{'))
                .discard_result()?;
            let ParseOk { result: body, state } = parse_Whitespace(state, tracer, cache)
                .and_then(|ParseOk { state, .. }| parse_DeclareBodyNode(state, tracer, cache))?;
            let ParseOk { state, .. } = parse_Whitespace(state, tracer, cache)
                .and_then(|ParseOk { state, .. }| parse_character_literal(state, '}'))
                .discard_result()?;
            Ok(ParseOk { result: Parsed { modifiers, id, body }, state })
        }
        pub struct Parsed {
            pub modifiers: Vec<ModifierNode>,
            pub id: IdentifierNode,
            pub body: DeclareBodyNode,
        }
        #[inline(always)]
        pub fn rule_parser<'a>(
            state: ParseState<'a>,
            tracer: impl ParseTracer,
            cache: &mut ParseCache<'a>,
        ) -> ParseResult<'a, super::DefineStatementNode> {
            let result = parse(state.clone(), tracer, cache)?.map_with_state(|r, new_state| super::DefineStatementNode {
                modifiers: r.modifiers,
                id: r.id,
                body: r.body,
                position: state.range_until(new_state),
            });
            Ok(result)
        }
    }
    #[inline]
    pub(super) fn parse_DefineStatementNode<'a>(
        state: ParseState<'a>,
        tracer: impl ParseTracer,
        cache: &mut ParseCache<'a>,
    ) -> ParseResult<'a, DefineStatementNode> {
        tracer.run_traced("DefineStatementNode", state, |state, tracer| {
            DefineStatementNode_impl::rule_parser(state, tracer, cache)
        })
    }
    impl PegPosition for DefineStatementNode {
        fn position(&self) -> &std::ops::Range<usize> {
            &self.position
        }
    }
    mod DeclareBodyNode_impl {
        use super::*;
        mod closure {
            use super::*;
        }
        #[inline(always)]
        pub fn parse<'a>(
            state: ParseState<'a>,
            tracer: impl ParseTracer,
            cache: &mut ParseCache<'a>,
        ) -> ParseResult<'a, Parsed> {
            let mut iterations: usize = 0;
            let mut state = state;
            let mut id: Vec<IdentifierNode> = Vec::new();
            loop {
                match parse_Whitespace(state.clone(), tracer, cache)
                    .and_then(|ParseOk { state, .. }| parse_IdentifierNode(state, tracer, cache))
                    .map_inner(|result| vec![result])
                {
                    Ok(ParseOk { result: __result, state: new_state, .. }) => {
                        id.extend(__result);
                        state = new_state;
                    }
                    Err(err) => {
                        state = state.record_error(err);
                        break;
                    }
                }
                iterations += 1;
            }
            Ok(ParseOk { result: id, state })
        }
        pub type Parsed = Vec<IdentifierNode>;
        #[inline(always)]
        pub fn rule_parser<'a>(
            state: ParseState<'a>,
            tracer: impl ParseTracer,
            cache: &mut ParseCache<'a>,
        ) -> ParseResult<'a, super::DeclareBodyNode> {
            let result = parse(state.clone(), tracer, cache)?
                .map_with_state(|r, new_state| super::DeclareBodyNode { id: r, position: state.range_until(new_state) });
            Ok(result)
        }
    }
    #[inline]
    pub(super) fn parse_DeclareBodyNode<'a>(
        state: ParseState<'a>,
        tracer: impl ParseTracer,
        cache: &mut ParseCache<'a>,
    ) -> ParseResult<'a, DeclareBodyNode> {
        tracer.run_traced("DeclareBodyNode", state, |state, tracer| DeclareBodyNode_impl::rule_parser(state, tracer, cache))
    }
    impl PegPosition for DeclareBodyNode {
        fn position(&self) -> &std::ops::Range<usize> {
            &self.position
        }
    }
    mod NumNode_impl {
        use super::*;
        mod closure {
            use super::*;
        }
        #[inline(always)]
        pub fn parse<'a>(
            state: ParseState<'a>,
            tracer: impl ParseTracer,
            cache: &mut ParseCache<'a>,
        ) -> ParseResult<'a, Parsed> {
            let mut iterations: usize = 0;
            let mut state = state;
            loop {
                match parse_character_range(state.clone(), '0', '9').discard_result() {
                    Ok(ParseOk { result: __result, state: new_state, .. }) => {
                        state = new_state;
                    }
                    Err(err) => {
                        state = state.record_error(err);
                        break;
                    }
                }
                iterations += 1;
            }
            if iterations == 0 {
                return Err(state.report_farthest_error());
            }
            Ok(ParseOk { result: (), state })
        }
        pub type Parsed = ();
        #[inline(always)]
        pub fn rule_parser<'a>(
            state: ParseState<'a>,
            tracer: impl ParseTracer,
            cache: &mut ParseCache<'a>,
        ) -> ParseResult<'a, NumNode> {
            let result = parse(state.clone(), tracer, cache)?.map_with_state(|_, new_state| {
                let string = state.slice_until(new_state).to_string();
                NumNode { string, position: state.range_until(new_state) }
            });
            Ok(result)
        }
    }
    #[inline]
    pub(super) fn parse_NumNode<'a>(
        state: ParseState<'a>,
        tracer: impl ParseTracer,
        cache: &mut ParseCache<'a>,
    ) -> ParseResult<'a, NumNode> {
        tracer.run_traced("NumNode", state, |state, tracer| NumNode_impl::rule_parser(state, tracer, cache))
    }
    impl PegPosition for NumNode {
        fn position(&self) -> &std::ops::Range<usize> {
            &self.position
        }
    }
    mod ModifierNode_impl {
        use super::*;
        mod part_1 {
            use super::*;
            mod negative_lookahead {
                use super::*;
                #[inline(always)]
                pub fn parse<'a>(
                    mut state: ParseState<'a>,
                    tracer: impl ParseTracer,
                    cache: &mut ParseCache<'a>,
                ) -> ParseResult<'a, Parsed> {
                    ChoiceHelper::new(state)
                        .choice(|state| {
                            parse_Whitespace(state, tracer, cache)
                                .and_then(|ParseOk { state, .. }| parse_character_literal(state, '{'))
                                .discard_result()
                        })
                        .choice(|state| {
                            parse_Whitespace(state, tracer, cache)
                                .and_then(|ParseOk { state, .. }| parse_character_literal(state, '['))
                                .discard_result()
                        })
                        .end()
                }
                pub type Parsed = ();
            }
            #[inline(always)]
            pub fn parse<'a>(
                state: ParseState<'a>,
                tracer: impl ParseTracer,
                cache: &mut ParseCache<'a>,
            ) -> ParseResult<'a, Parsed> {
                match negative_lookahead::parse(state.clone(), tracer, cache) {
                    Ok(_) => Err(state.report_error(ParseErrorSpecifics::NegativeLookaheadFailed)),
                    Err(_) => Ok(ParseOk { result: (), state }),
                }
            }
            pub type Parsed = ();
        }
        #[inline(always)]
        pub fn parse<'a>(
            state: ParseState<'a>,
            tracer: impl ParseTracer,
            cache: &mut ParseCache<'a>,
        ) -> ParseResult<'a, Parsed> {
            let ParseOk { state, .. } = parse_Whitespace(state, tracer, cache)
                .and_then(|ParseOk { state, .. }| parse_IdentifierNode(state, tracer, cache))
                .discard_result()?;
            let ParseOk { state, .. } = part_1::parse(state, tracer, cache)?;
            Ok(ParseOk { result: (), state })
        }
        pub type Parsed = ();
        #[inline(always)]
        pub fn rule_parser<'a>(
            state: ParseState<'a>,
            tracer: impl ParseTracer,
            cache: &mut ParseCache<'a>,
        ) -> ParseResult<'a, ModifierNode> {
            let result = parse(state.clone(), tracer, cache)?.map_with_state(|_, new_state| {
                let string = state.slice_until(new_state).to_string();
                ModifierNode { string, position: state.range_until(new_state) }
            });
            Ok(result)
        }
    }
    #[inline]
    pub(super) fn parse_ModifierNode<'a>(
        state: ParseState<'a>,
        tracer: impl ParseTracer,
        cache: &mut ParseCache<'a>,
    ) -> ParseResult<'a, ModifierNode> {
        tracer.run_traced("ModifierNode", state, |state, tracer| ModifierNode_impl::rule_parser(state, tracer, cache))
    }
    impl PegPosition for ModifierNode {
        fn position(&self) -> &std::ops::Range<usize> {
            &self.position
        }
    }
    mod IdentifierNode_impl {
        use super::*;
        mod part_1 {
            use super::*;
            mod closure {
                use super::*;
            }
            #[inline(always)]
            pub fn parse<'a>(
                state: ParseState<'a>,
                tracer: impl ParseTracer,
                cache: &mut ParseCache<'a>,
            ) -> ParseResult<'a, Parsed> {
                let mut iterations: usize = 0;
                let mut state = state;
                loop {
                    match parse_XID_CONTINUE(state.clone(), tracer, cache).discard_result() {
                        Ok(ParseOk { result: __result, state: new_state, .. }) => {
                            state = new_state;
                        }
                        Err(err) => {
                            state = state.record_error(err);
                            break;
                        }
                    }
                    iterations += 1;
                }
                Ok(ParseOk { result: (), state })
            }
            pub type Parsed = ();
        }
        #[inline(always)]
        pub fn parse<'a>(
            state: ParseState<'a>,
            tracer: impl ParseTracer,
            cache: &mut ParseCache<'a>,
        ) -> ParseResult<'a, Parsed> {
            let ParseOk { state, .. } = ChoiceHelper::new(state)
                .choice(|state| parse_XID_START(state, tracer, cache).discard_result())
                .choice(|state| parse_character_literal(state, '_').discard_result())
                .end()?;
            let ParseOk { state, .. } = part_1::parse(state, tracer, cache)?;
            Ok(ParseOk { result: (), state })
        }
        pub type Parsed = ();
        #[inline(always)]
        pub fn rule_parser<'a>(
            state: ParseState<'a>,
            tracer: impl ParseTracer,
            cache: &mut ParseCache<'a>,
        ) -> ParseResult<'a, IdentifierNode> {
            let result = parse(state.clone(), tracer, cache)?.map_with_state(|_, new_state| {
                let string = state.slice_until(new_state).to_string();
                IdentifierNode { string, position: state.range_until(new_state) }
            });
            Ok(result)
        }
    }
    #[inline]
    pub(super) fn parse_IdentifierNode<'a>(
        state: ParseState<'a>,
        tracer: impl ParseTracer,
        cache: &mut ParseCache<'a>,
    ) -> ParseResult<'a, IdentifierNode> {
        tracer.run_traced("IdentifierNode", state, |state, tracer| IdentifierNode_impl::rule_parser(state, tracer, cache))
    }
    impl PegPosition for IdentifierNode {
        fn position(&self) -> &std::ops::Range<usize> {
            &self.position
        }
    }
    #[inline]
    pub(super) fn parse_XID_START<'a>(
        state: ParseState<'a>,
        tracer: impl ParseTracer,
        cache: &mut ParseCache<'a>,
    ) -> ParseResult<'a, XID_START> {
        if let Some(c) = state.s().chars().next() {
            if !unicode_ident::is_xid_start(c) {
                return Err(state.report_error(ParseErrorSpecifics::ExpectedCharacterClass { name: "XID_START" }));
            }
        }
        else {
            return Err(state.report_error(ParseErrorSpecifics::ExpectedCharacterClass { name: "XID_START" }));
        }
        if let Ok(result) = parse_char(state.clone(), tracer, cache) {
            return Ok(result);
        }
        Err(state.report_error(ParseErrorSpecifics::ExpectedCharacterClass { name: "XID_START" }))
    }
    #[inline]
    pub(super) fn parse_XID_CONTINUE<'a>(
        state: ParseState<'a>,
        tracer: impl ParseTracer,
        cache: &mut ParseCache<'a>,
    ) -> ParseResult<'a, XID_CONTINUE> {
        if let Some(c) = state.s().chars().next() {
            if !unicode_ident::is_xid_continue(c) {
                return Err(state.report_error(ParseErrorSpecifics::ExpectedCharacterClass { name: "XID_CONTINUE" }));
            }
        }
        else {
            return Err(state.report_error(ParseErrorSpecifics::ExpectedCharacterClass { name: "XID_CONTINUE" }));
        }
        if let Ok(result) = parse_char(state.clone(), tracer, cache) {
            return Ok(result);
        }
        Err(state.report_error(ParseErrorSpecifics::ExpectedCharacterClass { name: "XID_CONTINUE" }))
    }
}
