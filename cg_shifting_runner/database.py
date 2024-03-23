from sqlalchemy import Boolean, create_engine, Column, Integer, String
from sqlalchemy.orm import sessionmaker
from sqlalchemy.orm import DeclarativeBase

# Define the SQLAlchemy engine
engine = create_engine('sqlite:///results.db', echo=False)


class Base(DeclarativeBase):
    pass


class Results(Base):
    __tablename__ = 'results'
    id = Column(Integer, primary_key=True)
    level_number = Column(Integer)
    level_input = Column(String)
    level_pass = Column(String)
    solved = Column(Boolean)
    solution = Column(String)


# Create the table in the database
Base.metadata.create_all(engine, checkfirst=True)

Session = sessionmaker(bind=engine)


def add_level(level_number, level_pass, level_input):
    with Session() as session:
        if _check_if_exist(level_number, session):
            return
        new_level = Results(level_number=level_number, level_pass=level_pass, solved=False, level_input=level_input)
        session.add(new_level)
        session.commit()


def set_solution(level_number, solution):
    with Session() as session:
        if _check_is_solved(level_number, session):
            return
        level = session.query(Results).filter(Results.level_number == level_number).first()
        level.solution = solution
        level.solved = True
        session.commit()


def get_first_unsolved():
    with Session() as session:
        level = session.query(Results).filter(Results.solved == False).first()
        return level


def _check_if_exist(level_number, session) -> bool:
    level = session.query(Results).filter(Results.level_number == level_number).first()
    return level is not None


def _check_is_solved(level_number, session) -> bool:
    level = session.query(Results).filter(Results.level_number == level_number).first()
    return level.solved