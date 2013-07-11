#[link(name="d_set", vers="1", author="jedestep")];
#[crate_type="lib"];

#[deriving(Eq)]
struct DSet<T> {
	value: T,
	parent: Option<@mut DSet<T>>,
	rank: int
}

impl<T:Eq> DSet<T> {
	fn new(val: T) -> @mut DSet<T> {
		let s = @mut DSet { value: val, parent: None, rank: 0 };
		s.parent = Some(s);
		s
	}

	fn find(@mut self) -> @mut DSet<T> {
		if self.parent.unwrap() == self {
			return self;
		}
		self.parent.unwrap().find()
	}

	fn union(@mut self, other: @mut DSet<T>) {
		let rs = self.find();
		let ro = other.find();
		if rs == ro { return; }
		
		if rs.rank < ro.rank { rs.parent = Some(ro); }
		else if rs.rank > ro.rank { ro.parent = Some(rs); }
		else { ro.parent = Some(rs); rs.rank += 1; }
	}
}
