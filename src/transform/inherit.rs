use super::fresh_var;
use crate::match_sharedterm;
use crate::term::{make, RichTerm, Term, UnaryOp};

pub fn transform_one(rt: RichTerm) -> RichTerm {
    match_sharedterm! {rt.term, with {
    Term::RecRecord(fields, dyn_fields, attrs, deps, inh) => {
        println!("inherit len: {}", inh.len());
        let mut fields = fields.clone();
        let renaming: Vec<(Vec<_>, Vec<_>, Option<RichTerm>)> = inh.into_iter().map(|(ids, rt)| {
            if rt.is_some() {
                // need a fresh var only for the record. a static access will be
                // performed on it.
                (ids, vec![fresh_var()], rt)
            } else {
                (ids.clone(), ids.iter().map(|_| fresh_var()).collect(), None)
            }
        }).collect();

        fields.extend(
            renaming
            .iter()
                .map(|(ids, vars, rt)| {
                    ids.iter().zip(vars).map(move |(id, var)| {
                        if rt.is_some() {
                            (
                                id.clone(),
                                make::op1(UnaryOp::StaticAccess(id.clone()), make::var(var.clone())),
                            )
                        } else {
                            (id.clone(), make::var(var.clone()))
                        }
                    })
                })
                .flatten(),
        );
        let rec = RichTerm::new(Term::RecRecord(fields, dyn_fields, attrs, deps, vec![]), rt.pos);
        renaming.iter().fold(rec, |rec, (ids, vars, rt)|
                      if let Some(rt) = rt {
                          make::let_in(vars[0].clone(), rt.clone(), rec)
                      } else {
                          ids.iter().zip(vars).fold(rec, |rec, (id, var)| make::let_in(var.clone(), make::var(id.clone()), rec))
                      })
    },
    } else  rt
    }
}
